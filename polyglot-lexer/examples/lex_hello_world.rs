// Example: Lex the minimal Hello World program
use polyglot_lexer::{lex, Token, TokenKind};

fn main() {
    let source = r#"[|] HelloWorld
[i] #None
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .message: pg\string << "Hello, World!"
[o] .message: pg\string
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 ===");
    println!("\nInput Program:");
    println!("----------------------------------------");
    println!("{}", source);
    println!("----------------------------------------\n");

    match lex(source) {
        Ok(tokens) => {
            println!("Token Stream ({} tokens):", tokens.len());
            println!("========================================\n");

            for (i, token) in tokens.iter().enumerate() {
                if token.kind != TokenKind::Eof {
                    println!(
                        "{:>3}. {:30} | {:20} @ line {:>2}, col {:>2}",
                        i + 1,
                        format!("{:?}", token.kind),
                        format!("\"{}\"", escape_lexeme(&token.lexeme)),
                        token.line,
                        token.column
                    );
                } else {
                    println!(
                        "{:>3}. {:30} | {:20} @ line {:>2}, col {:>2}",
                        i + 1,
                        format!("{:?}", token.kind),
                        "<end>",
                        token.line,
                        token.column
                    );
                }
            }

            println!("\n========================================");
            println!("✅ Lexing completed successfully!");

            // Summary
            let token_counts = count_token_types(&tokens);
            println!("\nToken Summary:");
            for (kind, count) in token_counts.iter() {
                println!("  - {:?}: {}", kind, count);
            }
        }
        Err(e) => {
            eprintln!("❌ Lexer Error: {}", e);
            eprintln!("   at line {}, column {}", e.line(), e.column());
        }
    }
}

fn escape_lexeme(lexeme: &str) -> String {
    lexeme
        .replace("\\", "\\\\")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
}

fn count_token_types(tokens: &[Token]) -> Vec<(String, usize)> {
    use std::collections::HashMap;

    let mut counts: HashMap<String, usize> = HashMap::new();
    for token in tokens {
        if token.kind != TokenKind::Eof {
            let key = format!("{:?}", token.kind);
            *counts.entry(key).or_insert(0) += 1;
        }
    }

    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));
    result
}
