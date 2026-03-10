// Example: Test [*] line continuation block marker
use polyglot_lexer::{lex, token::TokenKind};

fn main() {
    println!("=== Polyglot Lexer v0.0.2 - Line Continuation Test ===\n");

    // Test case 1: Basic line continuation
    let source1 = r#"[|] Test
[r] .x: pg\int << 42
[*]
[r] .y: pg\int << 43
[X]"#;

    println!("Test 1: Basic line continuation");
    println!("Source code:");
    println!("{}", source1);
    println!("\n{}", "=".repeat(60));

    match lex(source1) {
        Ok(tokens) => {
            println!("✓ Lexed successfully: {} tokens", tokens.len());

            // Check for BlockLineContinuation token
            let line_cont_count = tokens
                .iter()
                .filter(|t| t.kind == TokenKind::BlockLineContinuation)
                .count();

            // Check for Newline tokens
            let newline_count = tokens
                .iter()
                .filter(|t| t.kind == TokenKind::Newline)
                .count();

            println!("  [*] markers found: {}", line_cont_count);
            println!("  Newline tokens: {}", newline_count);

            // Show tokens around the [*] marker
            println!("\nTokens around [*] marker:");
            for (i, token) in tokens.iter().enumerate() {
                if token.kind == TokenKind::BlockLineContinuation {
                    // Show context: previous 2, current, next 2
                    for j in i.saturating_sub(2)..=(i + 2).min(tokens.len() - 1) {
                        let marker = if j == i { " ← [*] HERE" } else { "" };
                        println!(
                            "  [{}] {:?}: \"{}\" (line {}, col {}){}",
                            j,
                            tokens[j].kind,
                            tokens[j].lexeme,
                            tokens[j].line,
                            tokens[j].column,
                            marker
                        );
                    }
                }
            }

            if line_cont_count > 0 {
                println!("\n✅ [*] block marker recognized!");
            } else {
                println!("\n❌ [*] block marker not found!");
            }
        }
        Err(e) => {
            println!("❌ Lexer error: {}", e);
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("\nTest 2: Line continuation in assignment");

    let source2 = r#"[|] Test
[r] .x: pg\int << \
[*]
42
[X]"#;

    println!("Source code:");
    println!("{}", source2);
    println!();

    match lex(source2) {
        Ok(tokens) => {
            println!("✓ Lexed successfully: {} tokens", tokens.len());

            // Find the OpPush (<<) token and check what follows
            for (i, token) in tokens.iter().enumerate() {
                if token.kind == TokenKind::OpPushLeft {
                    println!("\nTokens after << operator:");
                    for j in i..=(i + 4).min(tokens.len() - 1) {
                        let marker = if tokens[j].kind == TokenKind::BlockLineContinuation {
                            " ← [*]"
                        } else if tokens[j].kind == TokenKind::Newline {
                            " ← NEWLINE"
                        } else {
                            ""
                        };
                        println!(
                            "  [{:2}] {:?}: \"{}\"{}",
                            j, tokens[j].kind, tokens[j].lexeme, marker
                        );
                    }
                    break;
                }
            }

            // Verify no Newline token between [*] and 42
            let mut found_continuation = false;
            let mut newline_after_continuation = false;

            for (i, token) in tokens.iter().enumerate() {
                if token.kind == TokenKind::BlockLineContinuation {
                    found_continuation = true;
                    // Check if next non-continuation token is Newline
                    if i + 1 < tokens.len() && tokens[i + 1].kind == TokenKind::Newline {
                        newline_after_continuation = true;
                    }
                }
            }

            if found_continuation && !newline_after_continuation {
                println!("\n✅ Line continuation working! Newline after [*] was skipped.");
            } else if !found_continuation {
                println!("\n❌ [*] marker not found!");
            } else {
                println!("\n❌ Newline was NOT skipped after [*]!");
            }
        }
        Err(e) => {
            println!("❌ Lexer error: {}", e);
        }
    }
}
