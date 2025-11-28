// Example: Lex conditional logic with advanced operators
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"// Conditional Logic Example
[@] Local@ConditionalExample:1.0.0
[#] 1
[X]

[|] BasicConditional
[i] .score: pg\int
[t] |T.Call
[W] |W.Polyglot.Scope

[?] .score >? 90
[~][r] .grade: pg\string << "A"
[~]

[?] .score >? 80
[~][r] .grade: pg\string << "B"
[~]

[?] *?
[~][r] .grade: pg\string << "F"
[~]

[o] .grade: pg\string
[X]

[|] BooleanLogic
[i] .age: pg\int
[i] .is_member: pg\bool
[t] |T.Call
[W] |W.Polyglot.Scope

[r] .discount: pg\int << 0

[?] .age >? 65
[&] .is_member =? #Boolean.True
[~][<] .discount << 20
[~]

[?] *?
[~][<] .discount << 0
[~]

[o] .discount: pg\int
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Conditional Logic Example ===");
    println!("\nInput Program:");
    println!("========================================");
    println!("{}", source);
    println!("========================================\n");

    match lex(source) {
        Ok(tokens) => {
            println!("Token Stream ({} tokens):", tokens.len());
            println!("========================================\n");

            // Highlight interesting token categories
            for (i, token) in tokens.iter().enumerate() {
                if token.kind == TokenKind::Eof {
                    println!(
                        "   {:>3}. {:30} | {:25} @ line {:>2}, col {:>2}",
                        i + 1,
                        format!("{:?}", token.kind),
                        "<end>",
                        token.line,
                        token.column
                    );
                    continue;
                }

                let highlight = match token.kind {
                    // Block markers
                    TokenKind::BlockPackageStart => "📦",
                    TokenKind::BlockVersionEnum => "🔢",
                    TokenKind::BlockConditional => "❓",
                    TokenKind::BlockBody => "📍",
                    TokenKind::BlockBoolAnd => "➕",
                    TokenKind::BlockBoolOr => "🔶",

                    // Comparison operators
                    TokenKind::OpEqual | TokenKind::OpNotEqual => "⚖️ ",
                    TokenKind::OpGreater | TokenKind::OpLess => "🔼",
                    TokenKind::OpGreaterEqual | TokenKind::OpLessEqual => "↕️ ",

                    // Pattern operators
                    TokenKind::OpWildcard | TokenKind::OpRegex => "🌟",

                    // Range operators
                    TokenKind::OpRangeClosed | TokenKind::OpRangeOpen |
                    TokenKind::OpRangeHalfLeft | TokenKind::OpRangeHalfRight => "📏",

                    // Reserved enums
                    TokenKind::ReservedBooleanTrue | TokenKind::ReservedBooleanFalse => "🔵",

                    // Comments
                    TokenKind::CommentSingle | TokenKind::CommentMulti => "💬",

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
            }

            println!("\n========================================");
            println!("✅ Lexing completed successfully!");

            // Analysis by category
            println!("\n📊 Token Analysis by Category:");

            let block_markers = tokens.iter().filter(|t| matches!(
                t.kind,
                TokenKind::BlockPackageStart | TokenKind::BlockVersionEnum |
                TokenKind::BlockPipelineStart | TokenKind::BlockInput |
                TokenKind::BlockTrigger | TokenKind::BlockWrapper |
                TokenKind::BlockSequential | TokenKind::BlockOutput |
                TokenKind::BlockEnd | TokenKind::BlockConditional |
                TokenKind::BlockBody | TokenKind::BlockBoolAnd |
                TokenKind::BlockInputBinding
            )).count();

            let comparison_ops = tokens.iter().filter(|t| matches!(
                t.kind,
                TokenKind::OpEqual | TokenKind::OpNotEqual |
                TokenKind::OpGreater | TokenKind::OpLess |
                TokenKind::OpGreaterEqual | TokenKind::OpLessEqual
            )).count();

            let pattern_ops = tokens.iter().filter(|t| matches!(
                t.kind,
                TokenKind::OpWildcard | TokenKind::OpRegex
            )).count();

            let reserved_enums = tokens.iter().filter(|t| matches!(
                t.kind,
                TokenKind::ReservedBooleanTrue | TokenKind::ReservedBooleanFalse |
                TokenKind::ReservedNone | TokenKind::ReservedPipelineNoInput |
                TokenKind::ReservedPgVarDeclared | TokenKind::ReservedPgVarReady |
                TokenKind::ReservedPgVarPending | TokenKind::ReservedPgVarFaulted |
                TokenKind::ReservedPgVarDefaultReady
            )).count();

            println!("  📦 Block Markers: {}", block_markers);
            println!("  ⚖️  Comparison Operators: {}", comparison_ops);
            println!("  🌟 Pattern Operators: {}", pattern_ops);
            println!("  🔵 Reserved Enumerations: {}", reserved_enums);
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
