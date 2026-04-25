use crate::lexer::token::{PolyglotToken, Spanned};
use crate::lexer::patterns::get_patterns;

pub fn lex(script: &str) -> Vec<Spanned<PolyglotToken>> {
    let mut tokens = Vec::new();
    let patterns = get_patterns();

    for (line_idx, line) in script.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let line_num = line_idx + 1;
        let mut col_idx = 0;

        // Phase 1: Indent (every 3 spaces is a TokIndent)
        let mut indent_spaces = 0;
        let mut has_tab = false;
        let mut chars = line.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c == ' ' {
                indent_spaces += 1;
                col_idx += 1;
                chars.next();
            } else if c == '\t' {
                has_tab = true;
                col_idx += 1;
                chars.next();
            } else {
                break;
            }
        }

        if has_tab {
            let invalid_str = line[0..col_idx].to_string();
            // A line must always dictate a scope, even if parsing the indentation failed.
            tokens.push(Spanned::new(PolyglotToken::Scope(0), line_num, 1));
            tokens.push(Spanned::new(PolyglotToken::IncorrectIndent(invalid_str), line_num, 1));
        } else {
            let indent_count = indent_spaces / 3;
            let remainder = indent_spaces % 3;
            
            // Exactly one Scope token per line representing the calculated hierarchy depth.
            tokens.push(Spanned::new(PolyglotToken::Scope(indent_count), line_num, 1));

            if remainder > 0 {
                // If there are 1 or 2 leftover spaces, it's an illegal indent depth
                tokens.push(Spanned::new(PolyglotToken::IncorrectIndent(" ".repeat(remainder)), line_num, (indent_count * 3) + 1));
            }
        }

        // The remaining expression string after indentation
        let mut expression = &line[col_idx..];

        // Phase 2: Marker Phase (Prefix extraction)
        if expression.starts_with("[-]") {
            tokens.push(Spanned::new(PolyglotToken::ActionExecSeq, line_num, col_idx + 1));
            expression = &expression[3..];
            col_idx += 3;
        } else if expression.starts_with("{#}") {
            tokens.push(Spanned::new(PolyglotToken::DefData, line_num, col_idx + 1));
            expression = &expression[3..];
            col_idx += 3;
        } else if expression.starts_with("[#]") {
            tokens.push(Spanned::new(PolyglotToken::ActionDataLoad, line_num, col_idx + 1));
            expression = &expression[3..];
            col_idx += 3;
        } else if !expression.trim().is_empty() {
            // A Polyglot line must have a structural marker leading the expression.
            // If we don't match one but the line isn't empty, we flag the gap and proceed.
            tokens.push(Spanned::new(PolyglotToken::MissingMarker, line_num, col_idx + 1));
        }

        // We clean any glue space before feeding to macro-matcher
        let trimmed_len = expression.len() - expression.trim_start().len();
        expression = expression.trim_start();
        col_idx += trimmed_len;
        
        // Phase 3: Expression Phase (Delegating to Pattern Registry)
        // By looping here, we allow multiple macros to be matched on the same line,
        // which powers our inline-comment parsing.
        while !expression.is_empty() {
            let mut matched = false;
            for pattern in &patterns {
                if let Some(caps) = pattern.regex.captures(expression) {
                    let extracted = (pattern.extractor)(&caps);
                    for t in extracted {
                        tokens.push(Spanned::new(t, line_num, col_idx + 1));
                    }
                    matched = true;
                    
                    // Advance the expression and col pointers by the length of the matched string
                    let match_len = caps.get(0).unwrap().end();
                    expression = &expression[match_len..];
                    col_idx += match_len;
                    
                    // Consume any glue spaces between parts
                    let trimmed_len = expression.len() - expression.trim_start().len();
                    expression = expression.trim_start();
                    col_idx += trimmed_len;
                    
                    break;
                }
            }

            if !matched {
                // Stabilized Fallback parsing: Unrecognized syntax gets swallowed as a contiguous block.
                let mut invalid_len = 0;
                for c in expression.chars() {
                    if c == ' ' { break; } // prevent swallowing separate elements
                    invalid_len += c.len_utf8();
                }
                if invalid_len == 0 {
                    invalid_len = expression.chars().next().unwrap().len_utf8();
                }

                let invalid_str = &expression[..invalid_len];
                tokens.push(Spanned::new(PolyglotToken::InvalidPattern(invalid_str.to_string()), line_num, col_idx + 1));
                
                expression = &expression[invalid_len..];
                col_idx += invalid_len;
                
                // Clean trailing spaces to prevent infinite loops on spaces if not matched
                let trimmed_len = expression.len() - expression.trim_start().len();
                expression = expression.trim_start();
                col_idx += trimmed_len;
            }
        }

        // Phase 4: Newline
        tokens.push(Spanned::new(PolyglotToken::TokNewline, line_num, line.chars().count() + 1));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_basic_pipeline() {
        let script = std::fs::read_to_string("tests/fixtures/basic_pipeline.pg").unwrap();
        let tokens = lex(&script);
        
        println!("\n=== Polyglot Token Stream ===");
        for t in &tokens {
            println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
        }
        println!("=============================\n");

        assert!(!tokens.is_empty(), "Token stream should not be empty");
    }

    #[test]
    fn test_lex_incorrect_indent() {
        let script = std::fs::read_to_string("tests/fixtures/incorrect_indent.pg").unwrap();
        let tokens = lex(&script);
        
        println!("\n=== Incorrect Indent Stream ===");
        for t in &tokens {
            println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
        }
        println!("===============================\n");
        
        // Assertions for exact coordinates to prove algorithmic safety
        assert_eq!(tokens[0].value, PolyglotToken::Scope(1));
        assert_eq!(tokens[1].value, PolyglotToken::IncorrectIndent(" ".to_string()));
        assert_eq!(tokens[1].col, 4); // The extra space
        
        assert_eq!(tokens[5].value, PolyglotToken::Scope(0)); // Start of line 2
        assert_eq!(tokens[6].value, PolyglotToken::IncorrectIndent("\t".to_string())); // The \t on line 2
    }

    #[test]
    fn test_lex_comments() {
        let script = std::fs::read_to_string("tests/fixtures/comments.pg").unwrap();
        let tokens = lex(&script);
        println!("\n=== Polyglot Comments Stream ===");
        for t in &tokens {
            println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
        }
        println!("================================\n");
    }

// Merged comment functionality into test_lex_comments fixture.
// Removing redundant inline string test test_lex_comment_nested_patterns.

    #[test]
    fn test_lex_edge_cases() {
        let script = std::fs::read_to_string("tests/fixtures/edge_cases.pg").unwrap();
        let tokens = lex(&script);
        println!("\n=== Polyglot Edge Cases Stream ===");
        for t in &tokens {
            println!("[L{:02}:C{:02}] {:?}", t.line, t.col, t.value);
        }
        println!("======================================\n");
        
        // Assert ghost line (L02) skip
        assert_eq!(tokens[0].line, 3);
        
        // Assert contiguous InvalidPattern slurp for @@@
        assert_eq!(tokens[3].value, PolyglotToken::InvalidPattern("@@@".to_string()));
        
        // Assert missing spaces triggers standalone and pushes invalid operators!
        assert_eq!(tokens[4].value, PolyglotToken::Variable("var".to_string()));
        
        // Since there is NO space, the algorithm contiguous-slurps the entire `<<#Config._database` block!
        assert_eq!(tokens[5].value, PolyglotToken::InvalidPattern("<<#Config._database".to_string()));
    }

    #[test]
    fn test_cli_execution() {
        use std::process::Command;
        use std::path::Path;
        use std::fs;

        let output_file = "tests/fixtures/cli_output_test.pgts";
        let _ = fs::remove_file(output_file); // Ensure clean state before start

        let status = Command::new("cargo")
            .args([
                "run", "--bin", "polyglot", "--",
                "--lexer",
                "-c", "tests/fixtures/basic_pipeline.pg",
                "-t", output_file
            ])
            .status()
            .expect("Failed to execute CLI binary via cargo");

        assert!(status.success(), "CLI command returned a non-zero exit code");
        assert!(Path::new(output_file).exists(), "CLI failed to generate the .pgts output file");

        let generated_content = fs::read_to_string(output_file).expect("Failed to read generated .pgts");
        
        // Assert that the CLI fundamentally parses and formats identically to the organic lex() core
        assert!(generated_content.contains("[L02:C01] ActionExecSeq"));
        assert!(generated_content.contains("Pipeline(\"Transform.Data\")"));
        
        // Cleanup organic test artifact
        let _ = fs::remove_file(output_file);
    }
}
