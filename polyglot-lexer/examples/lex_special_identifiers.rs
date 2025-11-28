// Example: Test special identifiers (DT, RT, TG, etc.)
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"[|] SpecialIdentifiersTest
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope

// Datetime operations (DT.*)
[r] .now: pg\dt << DT.Now
[r] .formatted: pg\string << DT.Format
[r] .parsed: pg\dt << DT.Parse

// Runtime wrappers (RT.*)
[r] .py_version: pg\string << RT.Python
[r] .node_version: pg\string << RT.Node
[r] .rust_version: pg\string << RT.Rust

// Trigger types (TG.*)
[r] .trigger_type: pg\string << TG.File
[r] .trigger_config: pg\string << TG.Timer

// Special wrapper identifiers
[r] |CallPythonPipeline
[W] |W.Python3.11

// Special trigger types
[t] |T.String.Call
[t] |T.File.Watch
[t] |T.Timer.Cron

[o] !NoError
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Special Identifiers Test ===\n");

    match lex(source) {
        Ok(tokens) => {
            // Categorize special identifier types
            let mut dt_identifiers = Vec::new();
            let mut rt_identifiers = Vec::new();
            let mut tg_identifiers = Vec::new();
            let mut trigger_types = Vec::new();
            let mut wrapper_types = Vec::new();

            for token in &tokens {
                match token.kind {
                    TokenKind::SpecialDatetime => dt_identifiers.push(token),
                    TokenKind::SpecialRuntime => rt_identifiers.push(token),
                    TokenKind::SpecialTrigger => tg_identifiers.push(token),
                    TokenKind::SpecialTriggerType => trigger_types.push(token),
                    TokenKind::SpecialWrapper => wrapper_types.push(token),
                    _ => {}
                }
            }

            println!("✅ Lexing completed successfully!");
            println!("   Total tokens: {}", tokens.len());
            println!();

            println!("📊 Special Identifier Analysis:");
            println!();

            println!("🕒 Datetime Operations (DT.*): {}", dt_identifiers.len());
            for tok in &dt_identifiers {
                println!("   • {} @ line {}", tok.lexeme, tok.line);
            }
            println!();

            println!("🔧 Runtime Wrappers (RT.*): {}", rt_identifiers.len());
            for tok in &rt_identifiers {
                println!("   • {} @ line {}", tok.lexeme, tok.line);
            }
            println!();

            println!("⚡ Trigger Types (TG.*): {}", tg_identifiers.len());
            for tok in &tg_identifiers {
                println!("   • {} @ line {}", tok.lexeme, tok.line);
            }
            println!();

            println!("🎯 Special Trigger Types (|T.*): {}", trigger_types.len());
            for tok in &trigger_types {
                println!("   • {} @ line {}", tok.lexeme, tok.line);
            }
            println!();

            println!("📦 Special Wrappers (|W.*): {}", wrapper_types.len());
            for tok in &wrapper_types {
                println!("   • {} @ line {}", tok.lexeme, tok.line);
            }
            println!();

            // Verification
            println!("🎯 Verification:");
            println!("   {} DT.* identifiers found", if dt_identifiers.len() >= 3 { "✓" } else { "✗" });
            println!("   {} RT.* identifiers found", if rt_identifiers.len() >= 3 { "✓" } else { "✗" });
            println!("   {} TG.* identifiers found", if tg_identifiers.len() >= 2 { "✓" } else { "✗" });
            println!("   {} |T.* trigger types found", if trigger_types.len() >= 3 { "✓" } else { "✗" });
            println!("   {} |W.* wrappers found", if wrapper_types.len() >= 2 { "✓" } else { "✗" });

            println!();
            if dt_identifiers.len() >= 3 && rt_identifiers.len() >= 3 &&
               tg_identifiers.len() >= 2 && trigger_types.len() >= 3 &&
               wrapper_types.len() >= 2 {
                println!("🎉 All special identifiers working correctly!");
            } else {
                println!("⚠️  Some special identifiers missing or not recognized");
            }
        }
        Err(e) => {
            eprintln!("❌ Lexer Error: {}", e);
            eprintln!("   at line {}, column {}", e.line(), e.column());
        }
    }
}
