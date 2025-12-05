// Example: Lex variable states with reserved namespaces
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"[@] Local@StateExample:1.0.0
[#] 1
[X]

[|] StateIntrospection
[i] .variable: pg\string
[t] |T.Call
[W] |W.Polyglot.Scope

[r] |ProcessData
[<] .input << .variable
[>] .output >> .result

// Reserved namespace: .*.pgvar.*
[r] .current_state: #PgVar.States << .result.pgvar.state
[r] .pending_time: pg\dt << .result.pgvar.history.Pending.at

[?] .result.pgvar.state =? #PgVar.States.Ready
[~][r] .ready_time: pg\dt << .result.pgvar.history.Ready.at
[~][o] .result: pg\string
[~]

[?] .result.pgvar.state =? #PgVar.States.Faulted
[~][r] .error_list: pg\array{!} << .result.pgvar.errors
[~][o] !NoError
[~]

[?] *?
[~][o] !NoError
[~]
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Variable States Example ===");
    println!("\nInput Program:");
    println!("========================================");
    println!("{}", source);
    println!("========================================\n");

    match lex(source) {
        Ok(tokens) => {
            println!("Token Stream ({} tokens):", tokens.len());
            println!("========================================\n");

            // Count tokens by category
            let mut reserved_count = 0;
            let mut error_identifiers = 0;
            let mut variable_identifiers = 0;
            let mut type_tokens = 0;
            let mut delimiters = 0;

            for (i, token) in tokens.iter().enumerate() {
                if token.kind == TokenKind::Eof {
                    continue;
                }

                // Categorize
                match token.kind {
                    TokenKind::ReservedPgVarDeclared
                    | TokenKind::ReservedPgVarDefaultReady
                    | TokenKind::ReservedPgVarPending
                    | TokenKind::ReservedPgVarReady
                    | TokenKind::ReservedPgVarFaulted
                    | TokenKind::ReservedBooleanTrue
                    | TokenKind::ReservedBooleanFalse
                    | TokenKind::ReservedNone
                    | TokenKind::ReservedPipelineNoInput => reserved_count += 1,

                    TokenKind::IdentifierError | TokenKind::ReservedNoError => {
                        error_identifiers += 1
                    }
                    TokenKind::IdentifierVariable => variable_identifiers += 1,

                    TokenKind::TypeNamespace
                    | TokenKind::TypeString
                    | TokenKind::TypeInt
                    | TokenKind::TypeFloat
                    | TokenKind::TypeBool
                    | TokenKind::TypeDatetime
                    | TokenKind::TypePath
                    | TokenKind::TypeSerial
                    | TokenKind::TypeArray
                    | TokenKind::TypeSet => type_tokens += 1,

                    TokenKind::DelimiterDot => delimiters += 1,

                    _ => {}
                }

                // Highlight specific tokens
                let highlight = match token.kind {
                    TokenKind::ReservedPgVarReady
                    | TokenKind::ReservedPgVarFaulted
                    | TokenKind::ReservedPgVarPending
                    | TokenKind::ReservedPgVarDeclared
                    | TokenKind::ReservedPgVarDefaultReady => "🔴",

                    TokenKind::IdentifierError | TokenKind::ReservedNoError => "⚠️ ",

                    TokenKind::IdentifierVariable if token.lexeme.contains("pgvar") => "🔐",
                    TokenKind::IdentifierVariable => "💎",

                    TokenKind::DelimiterDot if i > 0 && i < tokens.len() - 1 => {
                        // Check if this is part of reserved namespace access
                        if let Some(prev) = tokens.get(i - 1) {
                            if prev.kind == TokenKind::IdentifierVariable {
                                "🔗"
                            } else {
                                "  "
                            }
                        } else {
                            "  "
                        }
                    }

                    _ => "  ",
                };

                if token.kind != TokenKind::Newline && token.kind != TokenKind::Whitespace {
                    println!(
                        "{} {:>3}. {:30} | {:30} @ line {:>2}, col {:>2}",
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

            // Analysis
            println!("\n📊 Token Analysis:");
            println!(
                "  🔴 Reserved #PgVar.States enumerations: {}",
                reserved_count
            );
            println!(
                "  ⚠️  Error identifiers (!Error, !NoError): {}",
                error_identifiers
            );
            println!(
                "  💎 Variable identifiers (.variable): {}",
                variable_identifiers
            );
            println!("  🔗 Dot delimiters (namespace access): {}", delimiters);
            println!(
                "  🏷️  Type tokens (pg\\string, pg\\dt, etc.): {}",
                type_tokens
            );

            // Show sample of reserved namespace access
            println!("\n🔍 Reserved Namespace Access Pattern:");
            println!("  .result.pgvar.state");
            println!("  .result.pgvar.history.Pending.at");
            println!("  .result.pgvar.errors");
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
