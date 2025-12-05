// Example: Test all working operators
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"[|] OperatorTest
[i] .value: pg\int
[i] .name: pg\string
[i] .flag: pg\bool
[t] |T.Call
[W] |W.Polyglot.Scope

// Assignment operators
[r] .result1: pg\int << 42
[r] .result2: pg\int >> .value
[r] .result3: pg\int <~ 100

// Comparison operators
[?] .value =? 10
[~][r] .msg: pg\string << "equal"
[~]

[?] .value =!? 0
[~][r] .msg: pg\string << "not equal"
[~]

[?] .value >? 50
[~][r] .msg: pg\string << "greater"
[~]

[?] .value <? 100
[~][r] .msg: pg\string << "less"
[~]

[?] .value =>? 10
[~][r] .msg: pg\string << "greater or equal"
[~]

[?] .value =<? 100
[~][r] .msg: pg\string << "less or equal"
[~]

// Pattern operators
[?] .name *?
[~][r] .matched: pg\bool << #Boolean.True
[~]

[?] .name re? "^[A-Z]"
[~][r] .starts_upper: pg\bool << #Boolean.True
[~]

[?] *?
[~][r] .default: pg\bool << #Boolean.False
[~]

[o] .msg: pg\string
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Operator Test ===");
    println!("\nTesting: Assignment, Comparison, and Pattern Operators\n");

    match lex(source) {
        Ok(tokens) => {
            // Track operators
            let assignment_ops: Vec<_> = tokens
                .iter()
                .filter(|t| {
                    matches!(
                        t.kind,
                        TokenKind::OpPush | TokenKind::OpPull | TokenKind::OpDefault
                    )
                })
                .collect();

            let comparison_ops: Vec<_> = tokens
                .iter()
                .filter(|t| {
                    matches!(
                        t.kind,
                        TokenKind::OpEqual
                            | TokenKind::OpNotEqual
                            | TokenKind::OpGreater
                            | TokenKind::OpLess
                            | TokenKind::OpGreaterEqual
                            | TokenKind::OpLessEqual
                    )
                })
                .collect();

            let pattern_ops: Vec<_> = tokens
                .iter()
                .filter(|t| matches!(t.kind, TokenKind::OpWildcard | TokenKind::OpRegex))
                .collect();

            println!("✅ Lexing completed successfully!");
            println!("   Total tokens: {}", tokens.len());
            println!();

            println!("📊 Operator Breakdown:");
            println!();

            println!("⬅️  Assignment Operators ({} total):", assignment_ops.len());
            for tok in &assignment_ops {
                println!("   • {:?} \"{}\" @ line {}", tok.kind, tok.lexeme, tok.line);
            }
            println!();

            println!("⚖️  Comparison Operators ({} total):", comparison_ops.len());
            for tok in &comparison_ops {
                println!("   • {:?} \"{}\" @ line {}", tok.kind, tok.lexeme, tok.line);
            }
            println!();

            println!("🔍 Pattern Operators ({} total):", pattern_ops.len());
            for tok in &pattern_ops {
                println!("   • {:?} \"{}\" @ line {}", tok.kind, tok.lexeme, tok.line);
            }
            println!();

            // Verify all expected operators are present
            println!("🎯 Verification:");
            let has_push = assignment_ops.iter().any(|t| t.kind == TokenKind::OpPush);
            let has_pull = assignment_ops.iter().any(|t| t.kind == TokenKind::OpPull);
            let has_default = assignment_ops
                .iter()
                .any(|t| t.kind == TokenKind::OpDefault);
            let has_equal = comparison_ops.iter().any(|t| t.kind == TokenKind::OpEqual);
            let has_not_equal = comparison_ops
                .iter()
                .any(|t| t.kind == TokenKind::OpNotEqual);
            let has_greater = comparison_ops
                .iter()
                .any(|t| t.kind == TokenKind::OpGreater);
            let has_less = comparison_ops.iter().any(|t| t.kind == TokenKind::OpLess);
            let has_greater_eq = comparison_ops
                .iter()
                .any(|t| t.kind == TokenKind::OpGreaterEqual);
            let has_less_eq = comparison_ops
                .iter()
                .any(|t| t.kind == TokenKind::OpLessEqual);
            let has_wildcard = pattern_ops.iter().any(|t| t.kind == TokenKind::OpWildcard);
            let has_regex = pattern_ops.iter().any(|t| t.kind == TokenKind::OpRegex);

            println!("   Assignment:");
            println!("      {} << (push)", if has_push { "✓" } else { "✗" });
            println!("      {} >> (pull)", if has_pull { "✓" } else { "✗" });
            println!("      {} <~ (default)", if has_default { "✓" } else { "✗" });
            println!("   Comparison:");
            println!("      {} =? (equal)", if has_equal { "✓" } else { "✗" });
            println!(
                "      {} =!? (not equal)",
                if has_not_equal { "✓" } else { "✗" }
            );
            println!("      {} >? (greater)", if has_greater { "✓" } else { "✗" });
            println!("      {} <? (less)", if has_less { "✓" } else { "✗" });
            println!(
                "      {} =>? (greater or equal)",
                if has_greater_eq { "✓" } else { "✗" }
            );
            println!(
                "      {} =<? (less or equal)",
                if has_less_eq { "✓" } else { "✗" }
            );
            println!("   Pattern:");
            println!(
                "      {} *? (wildcard)",
                if has_wildcard { "✓" } else { "✗" }
            );
            println!("      {} re? (regex)", if has_regex { "✓" } else { "✗" });

            let all_present = has_push
                && has_pull
                && has_default
                && has_equal
                && has_not_equal
                && has_greater
                && has_less
                && has_greater_eq
                && has_less_eq
                && has_wildcard
                && has_regex;

            println!();
            if all_present {
                println!("🎉 All operators working perfectly!");
            } else {
                println!("⚠️  Some operators missing - check implementation");
            }
        }
        Err(e) => {
            eprintln!("❌ Lexer Error: {}", e);
            eprintln!("   at line {}, column {}", e.line(), e.column());
        }
    }
}
