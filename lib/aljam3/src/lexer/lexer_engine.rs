use crate::lexer::patterns::get_patterns;
use crate::lexer::token::{Aljam3Token, Spanned};

pub fn lex(script: &str) -> Vec<Spanned<Aljam3Token>> {
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
            tokens.push(Spanned::new(Aljam3Token::Scope(0), line_num, 1));
            tokens.push(Spanned::new(
                Aljam3Token::IncorrectIndent(invalid_str),
                line_num,
                1,
            ));
        } else {
            let indent_count = indent_spaces / 3;
            let remainder = indent_spaces % 3;

            // Exactly one Scope token per line representing the calculated hierarchy depth.
            tokens.push(Spanned::new(
                Aljam3Token::Scope(indent_count),
                line_num,
                1,
            ));

            if remainder > 0 {
                // If there are 1 or 2 leftover spaces, it's an illegal indent depth
                tokens.push(Spanned::new(
                    Aljam3Token::IncorrectIndent(" ".repeat(remainder)),
                    line_num,
                    (indent_count * 3) + 1,
                ));
            }
        }

        // The remaining expression string after indentation
        let mut expression = &line[col_idx..];

        // Phase 2: Marker Phase (Prefix extraction)
        let mut line_action: Option<Aljam3Token> = None;
        let mut matched_marker = false;

        let marker_map = [
            ("[-]", Aljam3Token::ActionExecSeq),
            ("{@}", Aljam3Token::DefPackage),
            ("{#}", Aljam3Token::DefData),
            ("[#]", Aljam3Token::ActionDataLoad),
            ("[@]", Aljam3Token::ActionImport),
            ("{-}", Aljam3Token::DefPipeline),
            ("(-)", Aljam3Token::PipelineIO),
            ("(#)", Aljam3Token::DataInput),
            ("(<)", Aljam3Token::InputParameterProperty),
            ("(>)", Aljam3Token::OutputParameterProperty),
            ("(=)", Aljam3Token::ExpanderIO),
            ("(*)", Aljam3Token::CollectorIO),
            ("(~)", Aljam3Token::ContinueIOLine),
            ("[~]", Aljam3Token::ContinueActionLine),
            ("[T]", Aljam3Token::ActionTrigger),
            ("[Q]", Aljam3Token::ActionQueue),
            ("[W]", Aljam3Token::ActionWrapper),
            ("[=]", Aljam3Token::ActionExecPar),
            ("[*]", Aljam3Token::ActionCollector),
            ("{T}", Aljam3Token::DefTrigger),
            ("{W}", Aljam3Token::DefWrapper),
            ("{N}", Aljam3Token::DefNative),
            ("{Q}", Aljam3Token::DefQueue),
            ("{!}", Aljam3Token::DefError),
            ("{_}", Aljam3Token::DefPermission),
            ("{*}", Aljam3Token::DefCollector),
            ("{$}", Aljam3Token::DefConstructor),
            ("[b]", Aljam3Token::ActionExecBg),
            ("[?]", Aljam3Token::ActionCondSwitch),
            ("[!]", Aljam3Token::ActionError),
            ("[.]", Aljam3Token::ActionDataAccessFixed),
            ("[:]", Aljam3Token::ActionDataAccessFlex),
            ("[&]", Aljam3Token::ActionLogicalAnd),
            ("[|]", Aljam3Token::ActionLogicalOr),
            ("[^]", Aljam3Token::ActionLogicalXor),
            ("[c]", Aljam3Token::ActionForeignCode),
            ("[C]", Aljam3Token::ActionForeignCode),
            ("[%]", Aljam3Token::ActionMetadata),
            ("[\\]", Aljam3Token::ActionScopeIn),
            ("[/]", Aljam3Token::ActionScopeOut),
        ];

        for (prefix, token) in marker_map.iter() {
            if expression.starts_with(prefix) {
                tokens.push(Spanned::new(token.clone(), line_num, col_idx + 1));
                line_action = Some(token.clone());
                expression = &expression[prefix.len()..];
                col_idx += prefix.len();
                matched_marker = true;
                break;
            }
        }

        if !matched_marker && !expression.trim().is_empty() {
            // A Aljam3 line must have a structural marker leading the expression.
            // If we don't match one, check if it's a known Phase-3-bound comment marker.
            if !(expression.starts_with("[ ]")
                || expression.starts_with("{ }")
                || expression.starts_with("( )"))
            {
                if expression.len() >= 3 && expression[..3].starts_with('{') && expression[..3].ends_with('}') {
                    tokens.push(Spanned::new(
                        Aljam3Token::InvalidDefinitionMarker(expression[..3].to_string()),
                        line_num,
                        col_idx + 1,
                    ));
                    expression = &expression[3..];
                    col_idx += 3;
                } else if expression.len() >= 3 && expression[..3].starts_with('[') && expression[..3].ends_with(']') {
                    tokens.push(Spanned::new(
                        Aljam3Token::InvalidActionMarker(expression[..3].to_string()),
                        line_num,
                        col_idx + 1,
                    ));
                    expression = &expression[3..];
                    col_idx += 3;
                } else if expression.len() >= 3 && expression[..3].starts_with('(') && expression[..3].ends_with(')') {
                    tokens.push(Spanned::new(
                        Aljam3Token::InvalidIOMarker(expression[..3].to_string()),
                        line_num,
                        col_idx + 1,
                    ));
                    expression = &expression[3..];
                    col_idx += 3;
                } else {
                    tokens.push(Spanned::new(
                        Aljam3Token::MissingMarker,
                        line_num,
                        col_idx + 1,
                    ));
                }
            }
        }

        // We clean any glue space before feeding to macro-matcher
        let trimmed_len = expression.len() - expression.trim_start().len();
        expression = expression.trim_start();
        col_idx += trimmed_len;

        if line_action == Some(Aljam3Token::ActionForeignCode) {
            if !expression.is_empty() {
                if trimmed_len == 0 {
                    tokens.push(Spanned::new(
                        Aljam3Token::InvalidPattern(expression.to_string()),
                        line_num,
                        col_idx + 1,
                    ));
                } else {
                    tokens.push(Spanned::new(
                        Aljam3Token::ForeignCode(expression.to_string()),
                        line_num,
                        col_idx + 1,
                    ));
                }
                col_idx += expression.len();
                expression = "";
            }
        }

        // Phase 3: Expression Phase (Delegating to Pattern Registry)
        // By looping here, we allow multiple macros to be matched on the same line,
        // which powers our inline-comment parsing.
        while !expression.is_empty() {
            let mut matched = false;

            for (prefix, _) in marker_map.iter() {
                if expression.starts_with(prefix) {
                    tokens.push(Spanned::new(
                        Aljam3Token::MisplacedMarker(prefix.to_string()),
                        line_num,
                        col_idx + 1,
                    ));
                    matched = true;
                    expression = &expression[prefix.len()..];
                    col_idx += prefix.len();
                    
                    let trimmed_len = expression.len() - expression.trim_start().len();
                    expression = expression.trim_start();
                    col_idx += trimmed_len;
                    break;
                }
            }

            if matched { continue; }

            for pattern in &patterns {
                if let Some(caps) = pattern.regex.captures(expression) {
                    let extracted = (pattern.extractor)(&caps, line_action.as_ref());
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
                    if c == ' ' {
                        break;
                    } // prevent swallowing separate elements
                    invalid_len += c.len_utf8();
                }
                if invalid_len == 0 {
                    invalid_len = expression.chars().next().unwrap().len_utf8();
                }

                let invalid_str = &expression[..invalid_len];
                tokens.push(Spanned::new(
                    Aljam3Token::UnknownAljam3Object(invalid_str.to_string()),
                    line_num,
                    col_idx + 1,
                ));

                expression = &expression[invalid_len..];
                col_idx += invalid_len;

                // Clean trailing spaces to prevent infinite loops on spaces if not matched
                let trimmed_len = expression.len() - expression.trim_start().len();
                expression = expression.trim_start();
                col_idx += trimmed_len;
            }
        }

        // Phase 4: Newline
        tokens.push(Spanned::new(
            Aljam3Token::TokNewline,
            line_num,
            line.chars().count() + 1,
        ));
    }

    tokens
}
