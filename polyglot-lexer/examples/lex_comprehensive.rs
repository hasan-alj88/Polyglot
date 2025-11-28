// Example: Comprehensive lexer test with all operators
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"[|] ComprehensiveTest
[i] .temperature: pg\int
[i] .email: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope

// Range operators: ?[, ?(, ?], ?)
[?] .temperature ?[20, 25]
[~][r] .status: pg\string << "comfortable"
[~]

[?] .temperature ?(25, 35)
[~][r] .status: pg\string << "warm"
[~]

[?] .temperature ?[0, 20)
[~][r] .status: pg\string << "cold"
[~]

// Regex pattern matching
[?] .email re? "^[a-zA-Z0-9]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,}$"
[~][r] .valid: pg\bool << #Boolean.True
[~]

[?] *?
[~][r] .valid: pg\bool << #Boolean.False
[~]

[o] .status: pg\string
[o] .valid: pg\bool
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Comprehensive Test ===");
    println!("\nInput Program:");
    println!("========================================");
    println!("{}", source);
    println!("========================================\n");

    match lex(source) {
        Ok(tokens) => {
            println!("Token Stream ({} tokens):", tokens.len());
            println!("========================================\n");

            // Track operator categories
            let mut range_ops = Vec::new();
            let mut comparison_ops = Vec::new();
            let mut pattern_ops = Vec::new();
            let mut assignment_ops = Vec::new();

            for (i, token) in tokens.iter().enumerate() {
                if token.kind == TokenKind::Eof {
                    continue;
                }

                // Categorize operators
                match token.kind {
                    TokenKind::OpRangeClosed | TokenKind::OpRangeOpen |
                    TokenKind::OpRangeHalfLeft | TokenKind::OpRangeHalfRight => {
                        range_ops.push((i + 1, token.clone()));
                    }

                    TokenKind::OpEqual | TokenKind::OpNotEqual |
                    TokenKind::OpGreater | TokenKind::OpLess |
                    TokenKind::OpGreaterEqual | TokenKind::OpLessEqual => {
                        comparison_ops.push((i + 1, token.clone()));
                    }

                    TokenKind::OpWildcard | TokenKind::OpRegex => {
                        pattern_ops.push((i + 1, token.clone()));
                    }

                    TokenKind::OpPush | TokenKind::OpPull | TokenKind::OpDefault => {
                        assignment_ops.push((i + 1, token.clone()));
                    }

                    _ => {}
                }

                // Highlight operators
                let highlight = match token.kind {
                    TokenKind::OpRangeClosed => "📏🔒",
                    TokenKind::OpRangeOpen => "📏🔓",
                    TokenKind::OpRangeHalfLeft => "📏◀️ ",
                    TokenKind::OpRangeHalfRight => "📏▶️ ",
                    TokenKind::OpRegex => "🔍",
                    TokenKind::OpWildcard => "🌟",
                    TokenKind::OpPush => "⬅️ ",
                    TokenKind::OpPull => "➡️ ",
                    TokenKind::OpDefault => "🔄",
                    TokenKind::ReservedBooleanTrue | TokenKind::ReservedBooleanFalse => "🔵",
                    _ => "  ",
                };

                if token.kind != TokenKind::Newline && token.kind != TokenKind::Whitespace {
                    println!(
                        "{} {:>3}. {:30} | {:35} @ line {:>2}, col {:>2}",
                        highlight,
                        i + 1,
                        format!("{:?}", token.kind),
                        format!("\"{}\"", escape_lexeme(&token.lexeme)),
                        token.line,
                        token.column
                    );
                }
            }

            println!("\n========================================");
            println!("✅ Lexing completed successfully!");

            // Detailed operator analysis
            println!("\n📊 Operator Analysis:");
            println!("\n📏 Range Operators ({} total):", range_ops.len());
            for (idx, tok) in &range_ops {
                println!("   #{}: {:?} \"{}\"", idx, tok.kind, tok.lexeme);
            }

            println!("\n⚖️  Comparison Operators ({} total):", comparison_ops.len());
            for (idx, tok) in &comparison_ops {
                println!("   #{}: {:?} \"{}\"", idx, tok.kind, tok.lexeme);
            }

            println!("\n🔍 Pattern Operators ({} total):", pattern_ops.len());
            for (idx, tok) in &pattern_ops {
                println!("   #{}: {:?} \"{}\"", idx, tok.kind, tok.lexeme);
            }

            println!("\n⬅️  Assignment Operators ({} total):", assignment_ops.len());
            for (idx, tok) in &assignment_ops {
                println!("   #{}: {:?} \"{}\"", idx, tok.kind, tok.lexeme);
            }

            println!("\n🎯 Summary:");
            println!("   Total tokens: {}", tokens.len());
            println!("   Range operators: {}", range_ops.len());
            println!("   Comparison operators: {}", comparison_ops.len());
            println!("   Pattern operators: {}", pattern_ops.len());
            println!("   Assignment operators: {}", assignment_ops.len());
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
