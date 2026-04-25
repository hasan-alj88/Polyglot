use crate::lexer::token::{PolyglotToken, Spanned};
use crate::lexer::patterns::get_patterns;

pub fn lex(script: &str) -> Vec<Spanned<PolyglotToken>> {
    let mut tokens = Vec::new();
    let patterns = get_patterns();

    for (line_idx, line) in script.lines().enumerate() {
        let line_num = line_idx + 1;
        let mut col_idx = 0;

        // Phase 1: Indent (every 3 spaces is a TokIndent)
        let mut indent_spaces = 0;
        let mut chars = line.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c == ' ' {
                indent_spaces += 1;
                chars.next();
                if indent_spaces == 3 {
                    tokens.push(Spanned::new(PolyglotToken::TokIndent, line_num, col_idx + 1));
                    indent_spaces = 0;
                }
                col_idx += 1;
            } else {
                break;
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
        }

        // We clean any glue space before feeding to macro-matcher
        let trimmed_len = expression.len() - expression.trim_start().len();
        expression = expression.trim_start();
        col_idx += trimmed_len;
        
        // Phase 3: Expression Phase (Delegating to Pattern Registry)
        if !expression.is_empty() {
            let mut matched = false;
            for pattern in &patterns {
                if let Some(caps) = pattern.regex.captures(expression) {
                    let extracted = (pattern.extractor)(&caps);
                    for t in extracted {
                        tokens.push(Spanned::new(t, line_num, col_idx + 1));
                    }
                    matched = true;
                    // For Lexer structural PoC, the macro consumes the whole statement string. 
                    break;
                }
            }

            if !matched {
                // Stabilized Fallback parsing: Unrecognized syntax gets swallowed, avoiding panic.
                for (i, c) in expression.chars().enumerate() {
                    tokens.push(Spanned::new(PolyglotToken::TokUnrecognized(c), line_num, col_idx + 1 + i));
                }
            }
        }

        // Phase 4: Newline
        tokens.push(Spanned::new(PolyglotToken::TokNewline, line_num, line.chars().count() + 1));
    }

    tokens
}
