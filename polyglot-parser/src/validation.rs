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

    // Step 5: Return results
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
[@] Local@TestPkg:1.0.0
[X]

[|] ValidPipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
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
[@] Local@TestPkg:1.0.0
[X]

[|] Pipeline
[i] .input: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope
[r] .value: pg\string << "unclosed string
[o] .result: pg\string
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
[@] Local@TestPkg:1.0.0
[X]

[|] Pipeline
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
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
[@] Local@TestPkg:1.0.0
[X]

[|] DuplicateName
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
[X]

[|] DuplicateName
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .value: pg\string
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
[@] Local@TestPkg:1.0.0
[X]

[|] First
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .result: pg\int
[X]

[|] First
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope
[o] .value: pg\string
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
}
