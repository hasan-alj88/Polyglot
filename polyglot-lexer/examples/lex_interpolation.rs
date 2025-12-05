// Example: Lex string with interpolation
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"[|] GreetUser
[i] .name: pg\string
[t] |T.Call
[W] |W.NoSetup.NoCleanup

[r] .greeting: pg\string << "Hello, {.name}!"
[r] .formatted: pg\string << "Count: {.num:Hex}"
[o] .greeting: pg\string
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - String Interpolation Example ===");
    println!("\nInput Program:");
    println!("----------------------------------------");
    println!("{}", source);
    println!("----------------------------------------\n");

    match lex(source) {
        Ok(tokens) => {
            println!("Token Stream ({} tokens):", tokens.len());
            println!("========================================\n");

            // Highlight string interpolation tokens
            for (i, token) in tokens.iter().enumerate() {
                if token.kind != TokenKind::Eof {
                    let highlight = match token.kind {
                        TokenKind::StringStart | TokenKind::StringEnd => "🔵",
                        TokenKind::StringContent => "📝",
                        TokenKind::InterpolationStart | TokenKind::InterpolationEnd => "🟢",
                        TokenKind::FormatIdentifier => "🟡",
                        _ => "  ",
                    };

                    println!(
                        "{} {:>3}. {:30} | {:25} @ line {:>2}, col {:>2}",
                        highlight,
                        i + 1,
                        format!("{:?}", token.kind),
                        format!("\"{}\"", escape_lexeme(&token.lexeme)),
                        token.line,
                        token.column
                    );
                } else {
                    println!(
                        "   {:>3}. {:30} | {:25} @ line {:>2}, col {:>2}",
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

            // Count string-related tokens
            println!("\n🔍 String Interpolation Token Analysis:");
            println!(
                "  🔵 StringStart/End: {}",
                tokens
                    .iter()
                    .filter(|t| matches!(t.kind, TokenKind::StringStart | TokenKind::StringEnd))
                    .count()
            );
            println!(
                "  📝 StringContent: {}",
                tokens
                    .iter()
                    .filter(|t| t.kind == TokenKind::StringContent)
                    .count()
            );
            println!(
                "  🟢 Interpolation Start/End: {}",
                tokens
                    .iter()
                    .filter(|t| matches!(
                        t.kind,
                        TokenKind::InterpolationStart | TokenKind::InterpolationEnd
                    ))
                    .count()
            );
            println!(
                "  🟡 Format Identifiers: {}",
                tokens
                    .iter()
                    .filter(|t| t.kind == TokenKind::FormatIdentifier)
                    .count()
            );
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
