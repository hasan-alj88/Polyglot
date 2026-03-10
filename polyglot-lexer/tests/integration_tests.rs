// Integration tests for v0.0.4 Lexer
// Tests complete examples and complex scenarios

use polyglot_lexer::{lex, TokenKind};

#[test]
fn test_pipeline_composition_example() {
    let source = r#"
[@] Local::ProcessFiles:1.0.0.0
[|] ProcessFiles
[t] TG.CLI
<trigger_input:pg.string << .input

[r] Step1
<input:pg.string << .trigger_input
>result:pg.string >> .step2_input

[|] |> Step2
>step2_result:pg.string >> .final_output

[X]
"#;

    let result = lex(source);
    assert!(result.is_ok(), "Lexing should succeed");

    let tokens = result.unwrap();

    // Verify key token types exist
    assert!(tokens.iter().any(|t| t.kind == TokenKind::OpPipelineCompose),
            "Should have pipeline composition operator |>");
    assert!(tokens.iter().any(|t| t.kind == TokenKind::IdentifierInputArgument),
            "Should have input parameters");
    assert!(tokens.iter().any(|t| t.kind == TokenKind::IdentifierOutputArgument),
            "Should have output parameters");
}

#[test]
fn test_loop_with_indentation() {
    let source = r#"
[~]
  .item << 1
  .item << 2
    .nested << 3
  .item << 4
[*]
"#;

    let result = lex(source);
    assert!(result.is_ok(), "Loop with indentation should lex");

    let tokens = result.unwrap();

    // Should have indentation tokens
    let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
    let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();

    assert!(indent_count > 0, "Should have INDENT tokens");
    assert!(dedent_count > 0, "Should have DEDENT tokens");
}

#[test]
fn test_all_v004_tokens_lex() {
    // Test that all v0.0.4 features lex correctly
    let source = r#"
[@] Local::Test:1.0.0.0
[|] TestPipeline

// Pipeline composition
[|] |> OtherPipeline

// Input/Output parameters
<input:pg.string << .value
>output:pg.int >> .result

// Metadata
%Doc "This is documentation"
%Author "Test Author"

// Operators
.x << .y
.x >> .y
.x <~ .y
.x ~> .y
.x << .y
.x >> .y

// Comparison
.x =? .y
.x >? .y
.x <? .y
.x >=? .y
.x <=? .y

// Loop markers
[~]
  .item << 1
[*]

[X]
"#;

    let result = lex(source);
    assert!(result.is_ok(), "All v0.0.4 features should lex: {:?}", result.err());
}

#[test]
fn test_large_file_performance() {
    // Generate a 1000-line file
    let mut source = String::from("[@] Local::Test:1.0.0.0\n[|] TestPipeline\n");

    for i in 0..1000 {
        source.push_str(&format!("[r] .var{} << \"value {}\"\n", i, i));
    }

    source.push_str("[X]\n");

    let start = std::time::Instant::now();
    let result = lex(&source);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "Large file should lex successfully");
    assert!(elapsed.as_millis() < 100,
            "1000-line file should lex in <100ms, took {}ms", elapsed.as_millis());
}

#[test]
fn test_error_messages_have_position() {
    // Test that errors include line and column information
    let source = "[z]"; // Invalid block marker

    let result = lex(source);
    assert!(result.is_err(), "Invalid marker should produce error");

    let error = result.unwrap_err();
    assert!(error.line() > 0, "Error should have line number");
    assert!(error.column() > 0, "Error should have column number");
}

#[test]
fn test_tabs_in_indentation_error() {
    let source = "[~]\n\t.item << 1\n[*]";

    let result = lex(source);
    assert!(result.is_err(), "Tabs in indentation should error");

    let error = result.unwrap_err();
    assert_eq!(error.line(), 2, "Error should be on line 2 (the tab line)");
}

#[test]
fn test_complex_nested_structure() {
    // Test multi-level indentation within a single loop body
    let source = r#"
[@] Local::Complex:1.0.0.0
[|] ComplexPipeline

[r] ProcessData
<input:pg.array << .data

[~]
  .item << .current
  [r] CheckItem
  <item:pg.string << .item
    .nested << .value
    [r] ProcessNested
  .item2 << .other
>processed:pg.string >> .result
[*]

>output:pg.array >> .final
[X]
"#;

    let result = lex(source);
    assert!(result.is_ok(), "Complex nested structure should lex: {:?}", result.err());

    let tokens = result.unwrap();

    // Verify we have multiple indentation levels
    let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
    assert!(indent_count >= 2, "Should have at least 2 levels of indentation");
}
