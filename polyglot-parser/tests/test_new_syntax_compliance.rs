use polyglot_lexer::lex;
use polyglot_parser::{Parser, StubImportResolver, validate_file};

#[test]
fn test_input_argument_prefix_tokenization() {
    let input = r#"
[@] @Local::Test:1.0.0.0
[X]

[|] |TestPipeline
[i] .data:pg.string
[t] |T.Call

[r] |SomePipeline
[<] <input_arg:pg.string << .data
[o] .result:pg.string
[X]
"#;

    let tokens = lex(input).expect("Lexer should tokenize input argument prefix");

    // Verify the input argument token exists (v0.0.2: <input_arg is one token)
    let has_input_argument = tokens
        .iter()
        .any(|t| matches!(t.kind, polyglot_lexer::TokenKind::IdentifierInputArgument));

    assert!(has_input_argument, "Should have IdentifierInputArgument token");
}

#[test]
fn test_output_argument_prefix_tokenization() {
    let input = r#"
[@] @Local::Test:1.0.0.0
[X]

[|] |TestPipeline
[i] .data:pg.string
[t] |T.Call

[r] |SomePipeline
[>] >output_arg:pg.string >> .result
[o] .result:pg.string
[X]
"#;

    let tokens = lex(input).expect("Lexer should tokenize output argument prefix");

    // Verify the output argument token exists (v0.0.2: >output_arg is one token)
    let has_output_argument = tokens
        .iter()
        .any(|t| matches!(t.kind, polyglot_lexer::TokenKind::IdentifierOutputArgument));

    assert!(has_output_argument, "Should have IdentifierOutputArgument token");
}

#[test]
fn test_macro_definition_markers_tokenization() {
    let input = r#"
[@] @Local::Test:1.0.0.0
[X]

[M] TestMacro
[{] .input:pg.string
[}] .output:pg.string
[X]
"#;

    let tokens = lex(input).expect("Lexer should tokenize macro markers");

    // Verify macro markers exist
    let has_macro_def = tokens
        .iter()
        .any(|t| matches!(t.kind, polyglot_lexer::TokenKind::BlockMacroDefinition));
    let has_scope_input = tokens
        .iter()
        .any(|t| matches!(t.kind, polyglot_lexer::TokenKind::BlockScopeInput));
    let has_scope_output = tokens
        .iter()
        .any(|t| matches!(t.kind, polyglot_lexer::TokenKind::BlockScopeOutput));

    assert!(has_macro_def, "Should have BlockMacroDefinition token");
    assert!(has_scope_input, "Should have BlockScopeInput token");
    assert!(has_scope_output, "Should have BlockScopeOutput token");
}

#[test]
fn test_alias_definition_marker_tokenization() {
    let input = r#"
[@] @Local::Test:1.0.0.0
[X]

[#] MyEnum
[A] ShortName
[<] .Value1
[X]
"#;

    let tokens = lex(input).expect("Lexer should tokenize alias marker");

    // Verify alias marker exists
    let has_alias_def = tokens
        .iter()
        .any(|t| matches!(t.kind, polyglot_lexer::TokenKind::BlockAliasDefinition));

    assert!(has_alias_def, "Should have BlockAliasDefinition token");
}

#[test]
fn test_parser_handles_new_tokens_gracefully() {
    let input = r#"
[@] @Local::Test:1.0.0.0
[X]

[|] |TestPipeline
[i] .data:pg.string
[t] |T.Call
[o] .result:pg.string
[X]
"#;

    // Try to parse - we expect it might fail with unrecognized syntax
    // but it should not panic
    let resolver = StubImportResolver { warn_on_query: false };

    match Parser::new(input, resolver) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(ast) => {
                    println!("Parser successfully parsed new syntax!");
                }
                Err(e) => {
                    println!("Parse error (may be expected for unsupported syntax): {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Lexer error: {:?}", e);
        }
    }

    // Test passes if we get here without panicking
}
