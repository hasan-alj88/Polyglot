// Example: Test string escape sequences
use polyglot_lexer::{lex, TokenKind};

fn main() {
    let source = r#"[|] EscapeSequenceTest
[i] #Pipeline.NoInput
[t] |T.Call
[W] |W.Polyglot.Scope

// Newline escape
[r] .newline: pg\string << "Line 1\nLine 2"

// Tab escape
[r] .tab: pg\string << "Column1\tColumn2"

// Quote escape
[r] .quote: pg\string << "He said \"Hello\""

// Backslash escape
[r] .backslash: pg\string << "Path: C:\\Users\\Name"

// Combined escapes
[r] .combined: pg\string << "Line 1\nTab:\tValue\n\"Quoted\""

[o] !NoError
[X]"#;

    println!("=== Polyglot Lexer v0.0.2 - Escape Sequence Test ===\n");

    match lex(source) {
        Ok(tokens) => {
            // Find string content tokens
            let string_contents: Vec<_> = tokens.iter()
                .filter(|t| t.kind == TokenKind::StringContent)
                .collect();

            println!("✅ Lexing completed successfully!");
            println!("   Total tokens: {}", tokens.len());
            println!();

            println!("📝 String Content Analysis:");
            println!("   Found {} string content tokens", string_contents.len());
            println!();

            for (i, tok) in string_contents.iter().enumerate() {
                println!("   String #{}: \"{}\" @ line {}", i + 1, escape_for_display(&tok.lexeme), tok.line);

                // Show what escapes are present
                let escapes = detect_escapes(&tok.lexeme);
                if !escapes.is_empty() {
                    println!("              Contains: {}", escapes.join(", "));
                }
                println!();
            }

            // Verify escape sequences are processed
            let has_newline = string_contents.iter().any(|t| t.lexeme.contains('\n'));
            let has_tab = string_contents.iter().any(|t| t.lexeme.contains('\t'));
            let has_quote = string_contents.iter().any(|t| t.lexeme.contains('"'));
            let has_backslash = string_contents.iter().any(|t| t.lexeme.contains('\\'));

            println!("🎯 Escape Sequence Verification:");
            println!("   {} \\n (newline) processed", if has_newline { "✓" } else { "✗" });
            println!("   {} \\t (tab) processed", if has_tab { "✓" } else { "✗" });
            println!("   {} \\\" (quote) processed", if has_quote { "✓" } else { "✗" });
            println!("   {} \\\\ (backslash) processed", if has_backslash { "✓" } else { "✗" });

            println!();
            if has_newline && has_tab && has_quote && has_backslash {
                println!("🎉 All escape sequences working correctly!");
            } else {
                println!("⚠️  Some escape sequences not processed");
                println!();
                println!("Note: If escapes are preserved as literals (e.g., '\\n' as two chars),");
                println!("      this may be expected behavior for the lexer.");
                println!("      The runtime may handle escape processing.");
            }
        }
        Err(e) => {
            eprintln!("❌ Lexer Error: {}", e);
            eprintln!("   at line {}, column {}", e.line(), e.column());
        }
    }
}

fn escape_for_display(s: &str) -> String {
    s.replace("\\", "\\\\")
     .replace("\n", "\\n")
     .replace("\t", "\\t")
     .replace("\r", "\\r")
     .replace("\"", "\\\"")
}

fn detect_escapes(s: &str) -> Vec<String> {
    let mut escapes = Vec::new();
    if s.contains('\n') { escapes.push("newline (\\n)".to_string()); }
    if s.contains('\t') { escapes.push("tab (\\t)".to_string()); }
    if s.contains('"') { escapes.push("quote (\\\")".to_string()); }
    if s.contains('\\') { escapes.push("backslash (\\\\)".to_string()); }
    escapes
}
