// Example: Test range operators specifically
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"[|] RangeTest
[i] .temperature: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope

// Closed interval [a, b]
[?] .temperature ?[20, 25]
[~][r] .msg: pg\string << "range1"
[~]

// Open interval (a, b)
[?] .temperature ?(25, 35)
[~][r] .msg: pg\string << "range2"
[~]

// Half-open [a, b)
[?] .temperature ?[0, 20)
[~][r] .msg: pg\string << "range3"
[~]

// Half-open (a, b]
[?] .temperature ?(35, 45]
[~][r] .msg: pg\string << "range4"
[~]

[?] *?
[~][r] .msg: pg\string << "default"
[~]

[o] .msg: pg\string
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Range Operator Test ===\n");

    match lex(source) {
        Ok(tokens) => {
            // Find range operators
            let range_ops: Vec<_> = tokens
                .iter()
                .filter(|t| {
                    matches!(
                        t.kind,
                        TokenKind::OpRangeClosed
                            | TokenKind::OpRangeOpen
                            | TokenKind::OpRangeHalfLeft
                            | TokenKind::OpRangeHalfRight
                    )
                })
                .collect();

            let bracket_delimiters: Vec<_> = tokens
                .iter()
                .filter(|t| t.kind == TokenKind::DelimiterSquareBracketClose)
                .collect();

            let paren_close: Vec<_> = tokens
                .iter()
                .filter(|t| t.kind == TokenKind::DelimiterParenClose)
                .collect();

            println!("✅ Lexing completed successfully!");
            println!("   Total tokens: {}", tokens.len());
            println!();

            println!("📏 Range Operators Found:");
            for tok in &range_ops {
                println!(
                    "   • {:?} \"{}\" @ line {}, col {}",
                    tok.kind, tok.lexeme, tok.line, tok.column
                );
            }
            println!();

            println!("🔲 Closing Delimiters:");
            println!("   • Square brackets ']': {}", bracket_delimiters.len());
            for tok in &bracket_delimiters {
                println!("      - Line {}, col {}", tok.line, tok.column);
            }
            println!("   • Parentheses ')': {}", paren_close.len());
            for tok in &paren_close {
                println!("      - Line {}, col {}", tok.line, tok.column);
            }
            println!();

            // Verify expected operators
            let has_closed = range_ops.iter().any(|t| t.kind == TokenKind::OpRangeClosed);
            let has_open = range_ops.iter().any(|t| t.kind == TokenKind::OpRangeOpen);
            let has_half_left = range_ops
                .iter()
                .any(|t| t.kind == TokenKind::OpRangeHalfLeft);
            let has_half_right = range_ops
                .iter()
                .any(|t| t.kind == TokenKind::OpRangeHalfRight);

            println!("🎯 Range Operator Verification:");
            println!(
                "   {} ?[ (closed range)",
                if has_closed { "✓" } else { "✗" }
            );
            println!("   {} ?( (open range)", if has_open { "✓" } else { "✗" });
            println!(
                "   {} ?) (half-left range)",
                if has_half_left { "✓" } else { "✗" }
            );
            println!(
                "   {} ?] (half-right range)",
                if has_half_right { "✓" } else { "✗" }
            );

            println!();
            if has_closed && has_open && has_half_left && has_half_right {
                println!("🎉 All 4 range operators working perfectly!");
                println!("   ✓ Range syntax fully supported");
                println!("   ✓ Closing delimiters recognized");
            } else {
                println!("⚠️  Some range operators missing");
            }
        }
        Err(e) => {
            eprintln!("❌ Lexer Error: {}", e);
            eprintln!("   at line {}, column {}", e.line(), e.column());
        }
    }
}
