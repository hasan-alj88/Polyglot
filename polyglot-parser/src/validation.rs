//! Syntax Validation API for Polyglot
//!
//! Provides standalone validation of `.pg` files without full compilation,
//! enabling fast feedback during development.
//!
//! # Example
//! ```ignore
//! use polyglot_parser::validate_file;
//! use std::path::Path;
//!
//! let path = Path::new("example.pg");
//! match validate_file(path) {
//!     Ok(()) => println!("File is valid!"),
//!     Err(errors) => {
//!         for error in errors {
//!             eprintln!("{}", error);
//!         }
//!     }
//! }
//! ```

use crate::ast::{Definition, Program};
use crate::import_resolver::StubImportResolver;
use crate::validation_error::{ErrorCategory, Severity, ValidationError};
use crate::Parser;
use polyglot_lexer::Lexer;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Validate a `.pg` file for syntax and basic semantic correctness
///
/// This function performs three levels of validation:
/// 1. **Lexical**: Ensures all tokens are valid
/// 2. **Syntactic**: Ensures the file parses correctly
/// 3. **Semantic**: Checks for duplicate pipeline names within the file
///
/// # Arguments
/// * `path` - Path to the `.pg` file to validate
///
/// # Returns
/// * `Ok(())` - File is valid
/// * `Err(Vec<ValidationError>)` - File has validation errors
///
/// # Performance
/// Designed to complete in <500ms for typical pipeline files
///
/// # Example
/// ```ignore
/// use polyglot_parser::validate_file;
/// use std::path::Path;
///
/// let result = validate_file(Path::new("my_pipeline.pg"));
/// assert!(result.is_ok());
/// ```
pub fn validate_file(path: &Path) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    let file_path_str = path.to_string_lossy().to_string();

    // Step 1: Read file
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            errors.push(ValidationError::new(
                Severity::Error,
                ErrorCategory::Io,
                format!("Failed to read file: {}", e),
                1,
                1,
            ).with_file_path(&file_path_str));
            return Err(errors);
        }
    };

    // Step 2: Lexer validation
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            errors.push(ValidationError::from_lexer_error(e, Some(file_path_str)));
            return Err(errors);
        }
    };

    // Step 3: Parser validation
    let resolver = StubImportResolver::new(); // Validation mode - no external dependencies
    let parser = match Parser::new_from_tokens(tokens, resolver) {
        Ok(parser) => parser,
        Err(e) => {
            // Parser construction error (shouldn't happen normally)
            errors.push(ValidationError::new(
                Severity::Error,
                ErrorCategory::Parser,
                format!("Parser initialization failed: {}", e),
                1,
                1,
            ).with_file_path(&file_path_str));
            return Err(errors);
        }
    };

    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            errors.push(ValidationError::from_parser_error(e, Some(file_path_str.clone())));
            // Don't return yet - we might still do partial semantic validation
            // But for now, if parsing fails, we can't do semantic checks
            return Err(errors);
        }
    };

    // Step 4: Semantic validation - check for duplicate pipelines
    validate_no_duplicate_pipelines(&program, &file_path_str, &mut errors);

    // Step 5: Semantic validation - check serial error handling (Safety Mechanism 4)
    validate_serial_error_handling(&program, &file_path_str, &mut errors);

    // Step 6: Semantic validation - check serial schema enforcement (Safety Mechanism 5)
    validate_serial_schema(&program, &file_path_str, &mut errors);

    // Step 7: Return results
    if errors.is_empty() {
        Ok(())
    } else {
        // Sort errors by line number for readability
        errors.sort_by_key(|e| (e.line, e.column));
        Err(errors)
    }
}

/// Check for duplicate pipeline definitions within a program
///
/// # Arguments
/// * `program` - Parsed program AST
/// * `file_path` - File path for error reporting
/// * `errors` - Error collection vector
fn validate_no_duplicate_pipelines(
    program: &Program,
    file_path: &str,
    errors: &mut Vec<ValidationError>,
) {
    let mut pipeline_names: HashMap<String, (usize, crate::span::Span)> = HashMap::new();

    for definition in &program.definitions {
        if let Definition::Pipeline(pipeline) = definition {
            let name = &pipeline.name;

            if let Some((count, first_span)) = pipeline_names.get_mut(name) {
                // Duplicate found!
                if *count == 1 {
                    // First duplicate - report both occurrences
                    errors.push(ValidationError::duplicate_pipeline(
                        name,
                        *first_span,
                        pipeline.span,
                        Some(file_path.to_string()),
                    ));
                } else {
                    // Additional duplicates
                    errors.push(ValidationError::new(
                        Severity::Error,
                        ErrorCategory::Semantic,
                        format!("Pipeline '{}' defined multiple times", name),
                        pipeline.span.start.line,
                        pipeline.span.start.column,
                    ).with_file_path(file_path));
                }
                *count += 1;
            } else {
                // First occurrence
                pipeline_names.insert(name.clone(), (1, pipeline.span));
            }
        }
    }
}

/// Check for serial blocks without error handling in enumerations
///
/// # Arguments
/// * `program` - Parsed program AST
/// * `file_path` - File path for error reporting
/// * `errors` - Error collection vector
fn validate_serial_error_handling(
    program: &Program,
    file_path: &str,
    errors: &mut Vec<ValidationError>,
) {
    use crate::ast::{Definition, Statement};

    for definition in &program.definitions {
        if let Definition::Enumeration(enumeration) = definition {
            // Check if enumeration has any [s] serial blocks
            let has_serial_blocks = enumeration.body.statements.iter().any(|stmt| {
                matches!(stmt, Statement::PipelineCall { .. })
            });

            if has_serial_blocks {
                // Check if enumeration has [!] error catch block
                let has_error_catch = enumeration.body.statements.iter().any(|stmt| {
                    matches!(stmt, Statement::ErrorCatch { .. })
                });

                if !has_error_catch {
                    // Missing error handling for serial blocks
                    let enum_name = enumeration.name.join("::");
                    errors.push(ValidationError::new(
                        Severity::Error,
                        ErrorCategory::Semantic,
                        format!(
                            "Enumeration '{}' has serial blocks [s] but no error handling [!]",
                            enum_name
                        ),
                        enumeration.span.start.line,
                        enumeration.span.start.column,
                    )
                    .with_file_path(file_path)
                    .with_suggestion("Add '[s][!] *' for default error handling or '[s][!]' with custom handler"));
                }
            }
        }
    }
}

/// Check for serial blocks without schema declarations in enumerations (Safety Mechanism 5)
///
/// # Arguments
/// * `program` - Parsed program AST
/// * `file_path` - File path for error reporting
/// * `errors` - Error collection vector
fn validate_serial_schema(
    program: &Program,
    file_path: &str,
    errors: &mut Vec<ValidationError>,
) {
    use crate::ast::Definition;

    for definition in &program.definitions {
        if let Definition::Enumeration(enumeration) = definition {
            // Check if enumeration has any [s] serial load blocks
            let has_serial_blocks = enumeration.body.statements.iter().any(|stmt| {
                matches!(stmt, crate::ast::Statement::PipelineCall { .. })
            });

            if has_serial_blocks {
                // Check if enumeration has schema declarations
                let has_schema = !enumeration.serial_schema.is_empty();

                if !has_schema {
                    // Missing schema for serial blocks
                    let enum_name = enumeration.name.join("::");
                    errors.push(ValidationError::new(
                        Severity::Error,
                        ErrorCategory::Semantic,
                        format!(
                            "Enumeration '{}' has serial blocks [s] but no schema declarations [~][s] <~",
                            enum_name
                        ),
                        enumeration.span.start.line,
                        enumeration.span.start.column,
                    )
                    .with_file_path(file_path)
                    .with_suggestion("Add '[~][s] <~ .field:type' declarations for expected fields or '[~][s] <~ *' for wildcard"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Helper to create a temporary .pg file for testing
    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        file
    }

    #[test]
    fn test_validate_valid_pipeline() {
        let content = r#"
[@] @Local::TestPkg:1.0.0.0
[X]

[|] ValidPipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg.int
[X]
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(result.is_ok(), "Valid pipeline should pass validation");
    }

    #[test]
    fn test_validate_lexer_error() {
        // Invalid token - unclosed string
        let content = r#"
[@] @Local::TestPkg:1.0.0.0
[X]

[|] Pipeline
[i] .input: pg.string
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .value: pg.string << "unclosed string
[o] .result: pg.string
[X]
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(result.is_err(), "Lexer error should be detected");
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].category, ErrorCategory::Lexer);
    }

    #[test]
    fn test_validate_parser_error() {
        // Missing [X] closing marker
        let content = r#"
[@] @Local::TestPkg:1.0.0.0
[X]

[|] Pipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg.int
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(result.is_err(), "Parser error should be detected");
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
        assert_eq!(errors[0].category, ErrorCategory::Parser);
    }

    #[test]
    fn test_validate_duplicate_pipelines() {
        let content = r#"
[@] @Local::TestPkg:1.0.0.0
[X]

[|] DuplicateName
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg.int
[X]

[|] DuplicateName
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .value: pg.string
[X]
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(result.is_err(), "Duplicate pipelines should be detected");
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].category, ErrorCategory::Semantic);
        assert!(errors[0].message.contains("DuplicateName"));
    }

    #[test]
    fn test_validate_nonexistent_file() {
        let result = validate_file(Path::new("/nonexistent/file.pg"));

        assert!(result.is_err(), "Nonexistent file should error");
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].category, ErrorCategory::Io);
    }

    #[test]
    fn test_error_sorting() {
        // Multiple errors should be sorted by line number
        let content = r#"
[@] @Local::TestPkg:1.0.0.0
[X]

[|] First
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg.int
[X]

[|] First
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .value: pg.string
[X]

[|] First
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .data: pg\bool
[X]
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(result.is_err());
        let errors = result.unwrap_err();

        // Errors should be sorted by line number
        for i in 1..errors.len() {
            assert!(errors[i].line >= errors[i - 1].line);
        }
    }

    // ============================================================================
    // Serial Error Handling Tests (Story 1.8)
    // ============================================================================

    #[test]
    fn test_validate_serial_enum_without_serial_blocks_passes() {
        // Enumeration without [s] blocks should not require error handling
        let content = r#"
[@] @Local::TestPkg:1.0.0.0
[X]

[#] #Config
[<] .timeout: pg.int << 30
[<] .api_key: pg.string << "default"
[X]
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        if let Err(ref errors) = result {
            eprintln!("Validation errors:");
            for error in errors {
                eprintln!("  - {:?}: {}", error.category, error.message);
            }
        }

        assert!(result.is_ok(), "Enumeration without serial blocks should pass validation");
    }

    #[test]
    fn test_validate_serial_enum_with_default_handler_passes() {
        // Enumeration with [s] block and [s][!] !* default handler should pass (v0.0.4)
        let content = r#"
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Config;Default
[A] #Config

[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .timeout:pg.int << .timeout
   [.] .api_key:pg.string << .api_key

[s][!] !*
{x}
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        if let Err(ref errors) = result {
            eprintln!("Validation errors:");
            for error in errors {
                eprintln!("  - {:?}: {}", error.category, error.message);
            }
        }

        assert!(result.is_ok(), "Enumeration with serial block and default handler should pass");
    }

    #[test]
    fn test_validate_serial_enum_with_custom_handler_passes() {
        // Enumeration with [s] block and [s][!] custom handler should pass (v0.0.4)
        let content = r#"
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Secrets;EnvSecrets
[A] #Secrets

[s] |YAML.Load"\\FileDir\\.env.secrets"
   [.] .api_key:pg.string << .api_key
   [.] .jwt_secret:pg.string << .jwt_secret

[s][!]
   [r] $error_count :pg.int << 1
{x}
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        if let Err(ref errors) = result {
            eprintln!("Validation errors:");
            for error in errors {
                eprintln!("  - {:?}: {}", error.category, error.message);
            }
        }

        assert!(result.is_ok(), "Enumeration with serial block and custom handler should pass");
    }

    #[test]
    fn test_validate_serial_enum_missing_handler_fails() {
        // Enumeration with [s] block but no error handler should fail (v0.0.4)
        let content = r#"
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Config;MissingHandler
[A] #Config

[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .timeout:pg.int << .timeout
   [.] .api_key:pg.string << .api_key

{x}
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(result.is_err(), "Enumeration with serial block but no handler should fail");
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1, "Should have exactly one validation error");
        assert_eq!(errors[0].category, ErrorCategory::Semantic);
        assert!(
            errors[0].message.contains("serial blocks [s]"),
            "Error message should mention serial blocks"
        );
        assert!(
            errors[0].message.contains("error handling"),
            "Error message should mention error handling"
        );
        assert!(
            errors[0].suggestion.is_some(),
            "Error should include a suggestion"
        );
    }

    #[test]
    fn test_validate_serial_enum_multiple_serial_blocks_with_handler_passes() {
        // Multiple [s] blocks with single error handler should pass (v0.0.4)
        let content = r#"
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #SystemConfig;MultiSource
[A] #SystemConfig

[s] |YAML.Load"\\FileDir\\database.yaml"
   [.] .database_url:pg.string << .url

[s] |YAML.Load"\\FileDir\\cache.yaml"
   [.] .cache_url:pg.string << .url

[s] |YAML.Load"\\FileDir\\queue.yaml"
   [.] .queue_url:pg.string << .url

[s][!] !*
{x}
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        if let Err(ref errors) = result {
            eprintln!("Validation errors:");
            for error in errors {
                eprintln!("  - {:?}: {}", error.category, error.message);
            }
        }

        assert!(
            result.is_ok(),
            "Enumeration with multiple serial blocks and handler should pass"
        );
    }

    #[test]
    fn test_validate_serial_enum_multiple_serial_blocks_missing_handler_fails() {
        // Multiple [s] blocks without error handler should fail (v0.0.4)
        let content = r#"
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #Config;MultiMissingHandler
[A] #Config

[s] |YAML.Load"\\FileDir\\database.yaml"
   [.] .db:pg.string << .connection

[s] |YAML.Load"\\FileDir\\cache.yaml"
   [.] .cache:pg.string << .connection

{x}
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(
            result.is_err(),
            "Enumeration with multiple serial blocks but no handler should fail"
        );
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].category, ErrorCategory::Semantic);
    }

    #[test]
    fn test_validate_serial_error_message_includes_enum_name() {
        // Error message should include the enumeration name (v0.0.4)
        let content = r#"
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #MySpecialConfig;Special
[A] #MySpecialConfig

[s] |JSON.Load"\\FileDir\\config.json"
   [.] .value:pg.string << .value

{x}
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors[0].message.contains("MySpecialConfig"),
            "Error message should include enumeration name 'MySpecialConfig'"
        );
    }

    #[test]
    fn test_validate_serial_multiple_enumerations_mixed() {
        // Test file with multiple enumerations - some valid, some invalid (v0.0.4)
        let content = r#"
{@} @Local:TestPkg:1.0.0.0
{x}

{#} #ValidConfig;AppConfig
[A] #ValidConfig

[s] |YAML.Load"\\FileDir\\config.yaml"
   [.] .timeout:pg.int << .timeout

[s][!] !*
{x}

{#} #InvalidConfig;AppSecrets
[A] #InvalidConfig

[s] |YAML.Load"\\FileDir\\secrets.yaml"
   [.] .api_key:pg.string << .api_key

{x}

{#} #NoSerialConfig;Debug
[A] #NoSerialConfig
{x}
        "#;

        let file = create_temp_file(content);
        let result = validate_file(file.path());

        assert!(
            result.is_err(),
            "File with invalid enumeration should fail validation"
        );
        let errors = result.unwrap_err();

        eprintln!("Got {} error(s):", errors.len());
        for error in &errors {
            eprintln!("  - {:?}: {}", error.category, error.message);
        }

        assert_eq!(
            errors.len(),
            1,
            "Should have exactly one error (InvalidConfig)"
        );
        assert!(errors[0].message.contains("InvalidConfig"));
    }
}
