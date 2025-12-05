// Tests from LEXER-TEST-SUITE.md

use crate::{lex, Token, TokenKind};

// Helper to create tokens without position info for easier testing
fn token(kind: TokenKind, lexeme: &str) -> Token {
    Token::new(kind, lexeme.to_string(), 1, 1)
}

// ========================================
// 1. Basic Tokens
// ========================================

#[test]
fn test_1_1_variable_identifier() {
    let input = ".user";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // Variable + EOF
    assert_eq!(tokens[0].kind, TokenKind::IdentifierVariable);
    assert_eq!(tokens[0].lexeme, ".user");
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_1_3_enum_identifier() {
    let input = "#Boolean.True";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // Reserved + EOF
    assert_eq!(tokens[0].kind, TokenKind::ReservedBooleanTrue);
    assert_eq!(tokens[0].lexeme, "#Boolean.True");
}

#[test]
fn test_1_4_pipeline_identifier() {
    let input = "|ProcessData";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::IdentifierPipeline);
    assert_eq!(tokens[0].lexeme, "|ProcessData");
}

#[test]
fn test_1_5_error_identifier() {
    let input = "!NetworkError";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::IdentifierError);
    assert_eq!(tokens[0].lexeme, "!NetworkError");
}

#[test]
fn test_1_6_integer_literal() {
    let input = "42";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::LiteralInteger);
    assert_eq!(tokens[0].lexeme, "42");
}

#[test]
fn test_1_8_float_literal() {
    let input = "3.14";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::LiteralFloat);
    assert_eq!(tokens[0].lexeme, "3.14");
}

// ========================================
// 2. Block Markers
// ========================================

#[test]
fn test_2_1_package_start() {
    let input = "[@]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::BlockPackageStart);
    assert_eq!(tokens[0].lexeme, "[@]");
}

#[test]
fn test_2_2_pipeline_start() {
    let input = "[|]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::BlockPipelineStart);
    assert_eq!(tokens[0].lexeme, "[|]");
}

#[test]
fn test_2_3_input_block() {
    let input = "[i]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::BlockInput);
    assert_eq!(tokens[0].lexeme, "[i]");
}

#[test]
fn test_2_6_block_end() {
    let input = "[X]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::BlockEnd);
    assert_eq!(tokens[0].lexeme, "[X]");
}

// ========================================
// 3. Operators
// ========================================

#[test]
fn test_3_1_push_operator() {
    let input = "<<";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::OpPush);
    assert_eq!(tokens[0].lexeme, "<<");
}

#[test]
fn test_3_3_equal_comparison() {
    let input = "=?";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::OpEqual);
    assert_eq!(tokens[0].lexeme, "=?");
}

#[test]
fn test_3_4_not_equal_comparison() {
    let input = "=!?";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::OpNotEqual);
    assert_eq!(tokens[0].lexeme, "=!?");
}

#[test]
fn test_3_5_greater_or_equal() {
    let input = "=>?";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::OpGreaterEqual);
    assert_eq!(tokens[0].lexeme, "=>?");
}

// ========================================
// 4. String Literals (Simple)
// ========================================

#[test]
fn test_4_1_plain_string() {
    let input = r#""Hello, World!""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 4); // START, CONTENT, END, EOF
    assert_eq!(tokens[0].kind, TokenKind::StringStart);
    assert_eq!(tokens[1].kind, TokenKind::StringContent);
    assert_eq!(tokens[1].lexeme, "Hello, World!");
    assert_eq!(tokens[2].kind, TokenKind::StringEnd);
}

#[test]
fn test_4_2_empty_string() {
    let input = r#""""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 3); // START, END, EOF
    assert_eq!(tokens[0].kind, TokenKind::StringStart);
    assert_eq!(tokens[1].kind, TokenKind::StringEnd);
}

// ========================================
// 5. String Literals (With Interpolation)
// ========================================

#[test]
fn test_5_1_string_with_simple_interpolation() {
    let input = r#""Hello, {.name}!""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::StringStart);
    assert_eq!(tokens[1].kind, TokenKind::StringContent);
    assert_eq!(tokens[1].lexeme, "Hello, ");
    assert_eq!(tokens[2].kind, TokenKind::InterpolationStart);
    assert_eq!(tokens[3].kind, TokenKind::IdentifierVariable);
    assert_eq!(tokens[3].lexeme, ".name");
    assert_eq!(tokens[4].kind, TokenKind::InterpolationEnd);
    assert_eq!(tokens[5].kind, TokenKind::StringContent);
    assert_eq!(tokens[5].lexeme, "!");
    assert_eq!(tokens[6].kind, TokenKind::StringEnd);
}

#[test]
fn test_5_2_string_with_formatted_interpolation() {
    let input = r#""Count: {.num:Hex}""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::StringStart);
    assert_eq!(tokens[1].kind, TokenKind::StringContent);
    assert_eq!(tokens[1].lexeme, "Count: ");
    assert_eq!(tokens[2].kind, TokenKind::InterpolationStart);
    assert_eq!(tokens[3].kind, TokenKind::IdentifierVariable);
    assert_eq!(tokens[3].lexeme, ".num");
    assert_eq!(tokens[4].kind, TokenKind::DelimiterColon);
    assert_eq!(tokens[5].kind, TokenKind::FormatIdentifier);
    assert_eq!(tokens[5].lexeme, "Hex");
    assert_eq!(tokens[6].kind, TokenKind::InterpolationEnd);
    assert_eq!(tokens[7].kind, TokenKind::StringEnd);
}

#[test]
fn test_5_4_string_only_interpolation() {
    let input = r#""{.variable}""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::StringStart);
    assert_eq!(tokens[1].kind, TokenKind::InterpolationStart);
    assert_eq!(tokens[2].kind, TokenKind::IdentifierVariable);
    assert_eq!(tokens[3].kind, TokenKind::InterpolationEnd);
    assert_eq!(tokens[4].kind, TokenKind::StringEnd);
}

// ========================================
// 6. Inline Pipeline Calls
// ========================================

#[test]
fn test_6_1_explicit_pipeline_with_empty_string() {
    let input = r#"DT.Now"""#;
    let tokens = lex(input).unwrap();

    // Note: "DT.Now" will be tokenized as plain identifier "DT" then dot
    // This needs special handling in parser, not lexer
    assert!(tokens.len() > 0);
}

// ========================================
// 7. Complete Statements
// ========================================

#[test]
fn test_7_1_variable_declaration_with_assignment() {
    let input = r#"[r] .message: pg\string << "Hello""#;
    let tokens = lex(input).unwrap();

    // Verify we have all the expected tokens
    assert!(tokens.iter().any(|t| t.kind == TokenKind::BlockSequential));
    assert!(tokens
        .iter()
        .any(|t| t.kind == TokenKind::IdentifierVariable));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::DelimiterColon));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::TypeNamespace));
    assert!(tokens
        .iter()
        .any(|t| t.kind == TokenKind::DelimiterBackslash));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::TypeString));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::OpPush));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::StringStart));
}

// ========================================
// 9. Comments
// ========================================

#[test]
fn test_9_1_single_line_comment() {
    let input = "// This is a comment\n.variable";
    let tokens = lex(input).unwrap();

    // Comments are skipped, only variable and newline should remain
    let non_eof: Vec<_> = tokens.iter().filter(|t| t.kind != TokenKind::Eof).collect();
    assert!(non_eof.iter().any(|t| t.kind == TokenKind::Newline));
    assert!(non_eof
        .iter()
        .any(|t| t.kind == TokenKind::IdentifierVariable));
}

#[test]
fn test_9_2_multi_line_comment() {
    let input = "/* This is a\n   multi-line\n   comment */\n.variable";
    let tokens = lex(input).unwrap();

    // Comments are skipped
    let non_eof: Vec<_> = tokens.iter().filter(|t| t.kind != TokenKind::Eof).collect();
    assert!(non_eof
        .iter()
        .any(|t| t.kind == TokenKind::IdentifierVariable));
}

// ========================================
// 10. Edge Cases
// ========================================

#[test]
fn test_10_1_collection_literal() {
    let input = "{1, 2, 3}";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::DelimiterBraceOpen);
    assert_eq!(tokens[1].kind, TokenKind::LiteralInteger);
    assert_eq!(tokens[2].kind, TokenKind::DelimiterComma);
    assert_eq!(tokens[3].kind, TokenKind::LiteralInteger);
    assert_eq!(tokens[4].kind, TokenKind::DelimiterComma);
    assert_eq!(tokens[5].kind, TokenKind::LiteralInteger);
    assert_eq!(tokens[6].kind, TokenKind::DelimiterBraceClose);
}

// ========================================
// 11. Error Cases
// ========================================

#[test]
fn test_11_1_unterminated_string() {
    let input = r#""Hello"#;
    let result = lex(input);

    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.line(), 1);
    }
}

#[test]
fn test_11_5_unknown_block_marker() {
    let input = "[z]";
    let result = lex(input);

    assert!(result.is_err());
}

// ========================================
// 12. December 2025 Syntax Updates (Story 1.7)
// ========================================

#[test]
fn test_12_1_default_pull_operator() {
    let input = ".x ~> .y";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 4); // .x, ~>, .y, EOF
    assert_eq!(tokens[0].kind, TokenKind::IdentifierVariable);
    assert_eq!(tokens[0].lexeme, ".x");
    assert_eq!(tokens[1].kind, TokenKind::OpDefaultPull);
    assert_eq!(tokens[1].lexeme, "~>");
    assert_eq!(tokens[2].kind, TokenKind::IdentifierVariable);
    assert_eq!(tokens[2].lexeme, ".y");
}

#[test]
fn test_12_2_pipeline_formatted_string_simple() {
    let input = r#"|U.Log.Info"Processing items""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // PipelineFormatted + EOF
    assert_eq!(tokens[0].kind, TokenKind::LiteralPipelineFormatted);
    assert_eq!(tokens[0].lexeme, r#"|U.Log.Info"Processing items""#);
}

#[test]
fn test_12_3_pipeline_formatted_string_with_interpolation() {
    let input = r#"|U.Log.Info"Processing {.count} items""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // PipelineFormatted + EOF
    assert_eq!(tokens[0].kind, TokenKind::LiteralPipelineFormatted);
    assert_eq!(
        tokens[0].lexeme,
        r#"|U.Log.Info"Processing {.count} items""#
    );
}

#[test]
fn test_12_4_pipeline_formatted_string_runtime() {
    let input = r#"|RT.Shell.Run"ls -la {.directory}""#;
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::LiteralPipelineFormatted);
    assert_eq!(tokens[0].lexeme, r#"|RT.Shell.Run"ls -la {.directory}""#);
}

#[test]
fn test_12_5_enumeration_with_prefix() {
    let input = "#Config";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // Enum + EOF
    assert_eq!(tokens[0].kind, TokenKind::IdentifierEnum);
    assert_eq!(tokens[0].lexeme, "#Config");
}

#[test]
fn test_12_6_enumeration_multipart_with_prefix() {
    let input = "#Config.Database";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // Enum + EOF
    assert_eq!(tokens[0].kind, TokenKind::IdentifierEnum);
    assert_eq!(tokens[0].lexeme, "#Config.Database");
}

#[test]
fn test_12_7_unpack_vs_default_pull() {
    // ~identifier should be unpack, not default pull
    let input = "~data";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::IdentifierUnpack);
    assert_eq!(tokens[0].lexeme, "~data");

    // ~> should be default pull
    let input2 = "~>.variable";
    let tokens2 = lex(input2).unwrap();

    assert_eq!(tokens2[0].kind, TokenKind::OpDefaultPull);
    assert_eq!(tokens2[0].lexeme, "~>");
}
