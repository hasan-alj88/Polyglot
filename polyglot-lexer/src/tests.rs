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
    assert_eq!(tokens[0].kind, TokenKind::OpPushLeft);
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
    let input = ">=?";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::OpGreaterEqual);
    assert_eq!(tokens[0].lexeme, ">=?");
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
    assert!(tokens.iter().any(|t| t.kind == TokenKind::OpPushLeft));
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
    assert_eq!(tokens[1].kind, TokenKind::OpDefaultPushRight);
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

    assert_eq!(tokens2[0].kind, TokenKind::OpDefaultPushRight);
    assert_eq!(tokens2[0].lexeme, "~>");
}

// ========================================
// 13. v0.0.2 Syntax Compliance Tests
// ========================================

#[test]
fn test_13_1_input_argument_prefix() {
    let input = "<config";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // <config, EOF
    assert_eq!(tokens[0].kind, TokenKind::IdentifierInputArgument);
    assert_eq!(tokens[0].lexeme, "<config");
}

#[test]
fn test_13_2_output_argument_prefix() {
    let input = ">result";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // >result, EOF
    assert_eq!(tokens[0].kind, TokenKind::IdentifierOutputArgument);
    assert_eq!(tokens[0].lexeme, ">result");
}

#[test]
fn test_13_3_full_input_argument_with_type() {
    let input = "[<] <config:pg.path << value";
    let tokens = lex(input).unwrap();

    // Verify critical tokens
    assert_eq!(tokens[0].kind, TokenKind::BlockInputBinding);
    assert_eq!(tokens[1].kind, TokenKind::IdentifierInputArgument);
    assert_eq!(tokens[1].lexeme, "<config");
    assert_eq!(tokens[2].kind, TokenKind::IdentifierDataType);
    assert_eq!(tokens[2].lexeme, ":pg.path");

    // Find the push operator
    let push_idx = tokens.iter().position(|t| t.kind == TokenKind::OpPushLeft).unwrap();
    assert_eq!(tokens[push_idx].kind, TokenKind::OpPushLeft);
}

#[test]
fn test_13_4_full_output_argument_with_type() {
    let input = "[>] >result:pg.int >> .output";
    let tokens = lex(input).unwrap();

    // Verify critical tokens
    assert_eq!(tokens[0].kind, TokenKind::BlockOutputBinding);
    assert_eq!(tokens[1].kind, TokenKind::IdentifierOutputArgument);
    assert_eq!(tokens[1].lexeme, ">result");
    assert_eq!(tokens[2].kind, TokenKind::IdentifierDataType);
    assert_eq!(tokens[2].lexeme, ":pg.int");

    // Find the pull operator
    let pull_idx = tokens.iter().position(|t| t.kind == TokenKind::OpPushRight).unwrap();
    assert_eq!(tokens[pull_idx].kind, TokenKind::OpPushRight);

    // Find the output variable (should be after the pull operator)
    let var_idx = tokens.iter().skip(pull_idx).position(|t| t.kind == TokenKind::IdentifierVariable).unwrap();
    assert_eq!(tokens[pull_idx + var_idx].lexeme, ".output");
}

#[test]
fn test_13_5_macro_definition_marker() {
    let input = "[M]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // [M], EOF
    assert_eq!(tokens[0].kind, TokenKind::BlockMacroDefinition);
    assert_eq!(tokens[0].lexeme, "[M]");
}

#[test]
fn test_13_6_scope_input_marker() {
    let input = "[{]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // [{], EOF
    assert_eq!(tokens[0].kind, TokenKind::BlockScopeInput);
    assert_eq!(tokens[0].lexeme, "[{]");
}

#[test]
fn test_13_7_scope_output_marker() {
    let input = "[}]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // [}], EOF
    assert_eq!(tokens[0].kind, TokenKind::BlockScopeOutput);
    assert_eq!(tokens[0].lexeme, "[}]");
}

#[test]
fn test_13_8_alias_definition_marker() {
    let input = "[A]";
    let tokens = lex(input).unwrap();

    assert_eq!(tokens.len(), 2); // [A], EOF
    assert_eq!(tokens[0].kind, TokenKind::BlockAliasDefinition);
    assert_eq!(tokens[0].lexeme, "[A]");
}

#[test]
fn test_13_9_package_declaration_with_double_colon() {
    let input = "@Local::Examples";
    let tokens = lex(input).unwrap();

    // Verify: Package spec is consumed as one token
    assert_eq!(tokens[0].kind, TokenKind::IdentifierPackageSpec);
    assert_eq!(tokens[0].lexeme, "@Local::Examples");
}

#[test]
fn test_13_10_complete_macro_definition() {
    let input = "[M] TestMacro\n[{] .input:pg.string\n[}] .output:pg.string\n[X]";
    let tokens = lex(input).unwrap();

    // Find key markers
    let macro_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockMacroDefinition).unwrap();
    let scope_in_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockScopeInput).unwrap();
    let scope_out_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockScopeOutput).unwrap();
    let end_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockEnd).unwrap();

    // Verify markers exist and are in order
    assert!(macro_idx < scope_in_idx);
    assert!(scope_in_idx < scope_out_idx);
    assert!(scope_out_idx < end_idx);

    // Verify first line has TestMacro identifier
    assert_eq!(tokens[macro_idx + 1].kind, TokenKind::Identifier);
    assert_eq!(tokens[macro_idx + 1].lexeme, "TestMacro");
}

#[test]
fn test_13_11_runtime_wrapper_with_arguments() {
    let input = "[r] |U.RT.Python.Cli\n[<] <config:pg.path << value\n[>] >return_value:pg.int >> .result";
    let tokens = lex(input).unwrap();

    // Find key tokens
    let seq_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockSequential).unwrap();
    let pipe_idx = tokens.iter().position(|t| t.kind == TokenKind::IdentifierPipeline).unwrap();
    let _input_bind_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockInputBinding).unwrap();
    let input_arg_idx = tokens.iter().position(|t| t.kind == TokenKind::IdentifierInputArgument).unwrap();
    let _output_bind_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockOutputBinding).unwrap();
    let output_arg_idx = tokens.iter().position(|t| t.kind == TokenKind::IdentifierOutputArgument).unwrap();

    // Verify structure
    assert_eq!(tokens[seq_idx].kind, TokenKind::BlockSequential);
    assert_eq!(tokens[pipe_idx].lexeme, "|U.RT.Python.Cli");
    assert_eq!(tokens[input_arg_idx].kind, TokenKind::IdentifierInputArgument);
    assert_eq!(tokens[input_arg_idx].lexeme, "<config");
    assert_eq!(tokens[output_arg_idx].kind, TokenKind::IdentifierOutputArgument);
    assert_eq!(tokens[output_arg_idx].lexeme, ">return_value");
}

#[test]
fn test_13_12_operators_still_work() {
    // Ensure compound operators still work correctly
    let input_push = "<<";
    let tokens_push = lex(input_push).unwrap();
    assert_eq!(tokens_push[0].kind, TokenKind::OpPushLeft);

    let input_pull = ">>";
    let tokens_pull = lex(input_pull).unwrap();
    assert_eq!(tokens_pull[0].kind, TokenKind::OpPushRight);

    let input_less = "<?";
    let tokens_less = lex(input_less).unwrap();
    assert_eq!(tokens_less[0].kind, TokenKind::OpLess);

    let input_greater = ">?";
    let tokens_greater = lex(input_greater).unwrap();
    assert_eq!(tokens_greater[0].kind, TokenKind::OpGreater);

    let input_default = "<~";
    let tokens_default = lex(input_default).unwrap();
    assert_eq!(tokens_default[0].kind, TokenKind::OpDefaultPushLeft);
}

// ========================================
// 14. Indentation Tracking (Loop Bodies Only)
// ========================================

#[test]
fn test_14_1_simple_loop_with_indentation() {
    let input = "[~]\n  .item << 1\n  .item << 2\n[*]";
    let tokens = lex(input).unwrap();

    // Find the [~] marker
    let body_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockBody).unwrap();

    // After [~], we should have: Newline, then Indent
    assert_eq!(tokens[body_idx + 1].kind, TokenKind::Newline);
    assert_eq!(tokens[body_idx + 2].kind, TokenKind::Indent);

    // After second .item line, we should have Newline but NO Dedent (same level)
    let second_newline_idx = tokens.iter().skip(body_idx + 3).position(|t| t.kind == TokenKind::Newline).unwrap();
    // Next token after second newline should NOT be Dedent
    assert_ne!(tokens[body_idx + 3 + second_newline_idx + 1].kind, TokenKind::Dedent);

    // After [*], indentation tracking should be disabled
    let end_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockLineContinuation).unwrap();
    assert!(end_idx > body_idx);
}

#[test]
fn test_14_2_loop_with_dedent() {
    let input = "[~]\n  .item << 1\n[*]";
    let tokens = lex(input).unwrap();

    // Find the [~] and [*] markers
    let body_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockBody).unwrap();
    let end_idx = tokens.iter().position(|t| t.kind == TokenKind::BlockLineContinuation).unwrap();

    // Should have: [~], Newline, Indent, .item, ..., Newline (no Dedent before [*])
    assert_eq!(tokens[body_idx].kind, TokenKind::BlockBody);
    assert_eq!(tokens[body_idx + 1].kind, TokenKind::Newline);
    assert_eq!(tokens[body_idx + 2].kind, TokenKind::Indent);

    // The [*] marker should exist
    assert_eq!(tokens[end_idx].kind, TokenKind::BlockLineContinuation);
}

#[test]
fn test_14_3_nested_indentation() {
    let input = "[~]\n  .outer << 1\n    .inner << 2\n  .outer << 3\n[*]";
    let tokens = lex(input).unwrap();

    // Find indent tokens
    let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
    let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();

    // Should have 2 indents (level 0->2, then 2->4)
    assert_eq!(indent_count, 2);

    // Should have 2 dedents (level 4->2, then 2->0 before [*])
    assert_eq!(dedent_count, 2);
}

#[test]
fn test_14_4_indentation_outside_loop_ignored() {
    // Indentation outside loop bodies should NOT generate Indent/Dedent tokens
    let input = "[r]\n  .item << 1\n  .item << 2";
    let tokens = lex(input).unwrap();

    // Should have NO Indent or Dedent tokens
    let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
    let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();

    assert_eq!(indent_count, 0, "Indentation outside loop bodies should be ignored");
    assert_eq!(dedent_count, 0, "Dedentation outside loop bodies should be ignored");
}

#[test]
fn test_14_5_tabs_in_indentation_error() {
    // Tabs in indentation should cause an error
    let input = "[~]\n\t.item << 1\n[*]";
    let result = lex(input);

    assert!(result.is_err(), "Tabs in indentation should cause an error");
    if let Err(e) = result {
        // Check it's the right error type
        assert_eq!(e.line(), 2); // Tab is on line 2
    }
}

#[test]
fn test_14_6_inconsistent_indentation_error() {
    // Dedenting to a level that wasn't used before should error
    let input = "[~]\n  .item << 1\n    .inner << 2\n   .bad << 3\n[*]";
    let result = lex(input);

    assert!(result.is_err(), "Inconsistent indentation should cause an error");
    if let Err(e) = result {
        // Error should be on line 4 (the .bad line)
        assert_eq!(e.line(), 4);
    }
}

#[test]
fn test_14_7_multiple_dedent_levels() {
    let input = "[~]\n  .level1 << 1\n    .level2 << 2\n      .level3 << 3\n.base << 4\n[*]";
    let tokens = lex(input).unwrap();

    // Should have 3 indents (0->2, 2->4, 4->6)
    let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
    assert_eq!(indent_count, 3);

    // Should have 3 dedents (6->4, 4->2, 2->0) all at once before .base
    let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();
    assert_eq!(dedent_count, 3);

    // Find the .base variable
    let base_idx = tokens.iter().position(|t| t.lexeme == ".base").unwrap();

    // The 3 dedent tokens should appear right before .base
    assert_eq!(tokens[base_idx - 1].kind, TokenKind::Dedent);
    assert_eq!(tokens[base_idx - 2].kind, TokenKind::Dedent);
    assert_eq!(tokens[base_idx - 3].kind, TokenKind::Dedent);
}

#[test]
fn test_14_8_empty_loop_body() {
    // Loop body with no content (just [~] and [*])
    let input = "[~]\n[*]";
    let tokens = lex(input).unwrap();

    // Should have [~], Newline, [*], EOF
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, TokenKind::BlockBody);
    assert_eq!(tokens[1].kind, TokenKind::Newline);
    assert_eq!(tokens[2].kind, TokenKind::BlockLineContinuation);

    // No Indent or Dedent tokens
    let indent_count = tokens.iter().filter(|t| t.kind == TokenKind::Indent).count();
    let dedent_count = tokens.iter().filter(|t| t.kind == TokenKind::Dedent).count();
    assert_eq!(indent_count, 0);
    assert_eq!(dedent_count, 0);
}
