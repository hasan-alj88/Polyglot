use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet, get_def_target_help};
use super::Rule;

pub struct LexicalStructuralAlgorithm;

impl Rule for LexicalStructuralAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        let mut current_scope = 0;
        
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let token_val = &spanned_token.value;
            let line = spanned_token.line;
            let col = spanned_token.col;

            if let PolyglotToken::Scope(s) = token_val {
                current_scope = *s;
            }

            match token_val {
                // PGE01041
                PolyglotToken::MissingMarker => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01041".to_string(),
                        name: "Missing Structural Marker".to_string(),
                        message: "Line lacks a valid structural starting bracket.".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Ensure the line starts with a valid structural marker like `[-]`, `{-}`, or `[ ]`.".to_string())
                    });
                }
                // PGE01042
                PolyglotToken::IncorrectIndent(s) => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01042".to_string(),
                        name: "Incorrect Indentation Multiple".to_string(),
                        message: format!("Indentation must be a multiple of 3. Found `{}`.", s),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Polyglot uses strict 3-space indentation to define scope. Adjust spaces to a multiple of 3.".to_string())
                    });
                }
                // PGE01043
                PolyglotToken::InvalidPattern(s) => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01043".to_string(),
                        name: "Invalid Character Pattern".to_string(),
                        message: format!("Found an invalid character or unrecognizable pattern: `{}`", s),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Check the syntax around this character. It does not match any valid Polyglot lexer token.".to_string())
                    });
                }
                // PGE01044
                PolyglotToken::InvalidDefinitionMarker(s) => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01044".to_string(),
                        name: "Unrecognized Definition Marker".to_string(),
                        message: format!("Unknown character found inside definition curly braces: `{}`", s),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Valid definition markers include `{@}`, `{#}`, `{-}`, `{T}`, etc. Check the Polyglot specification for valid markers.".to_string())
                    });
                }
                // PGE01045
                PolyglotToken::InvalidActionMarker(s) => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01045".to_string(),
                        name: "Unrecognized Action Marker".to_string(),
                        message: format!("Unknown character found inside action square brackets: `{}`", s),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Valid action markers include `[-]`, `[=]`, `[@]`, `[T]`, etc. Check the Polyglot specification.".to_string())
                    });
                }
                // PGE01046
                PolyglotToken::InvalidIOMarker(s) => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01046".to_string(),
                        name: "Unrecognized IO Marker".to_string(),
                        message: format!("Unknown character found inside IO parentheses: `{}`", s),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Valid IO markers include `(-)`, `(#)`, `(<)`, `(>)`, etc. Check the Polyglot specification.".to_string())
                    });
                }
                // PGE01047
                PolyglotToken::UnknownPolyglotObject(s) => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01047".to_string(),
                        name: "Unknown Polyglot Object".to_string(),
                        message: format!("Identifier has an unknown prefix or invalid structure: `{}`", s),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Ensure your identifiers use the correct sigil (e.g. `$` for variables, `#` for data, `-` for pipelines).".to_string())
                    });
                }
                // PGE01054
                PolyglotToken::MisplacedMarker(m) => {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01054".to_string(),
                        name: "Misplaced Structural Marker".to_string(),
                        message: format!("Structural marker `{}` found mid-expression. Markers must be placed at the start of the line, exactly after the Scope indentation.", m),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Move this marker to a new line, ensuring it follows the correct indentation for its intended scope.".to_string())
                    });
                }
                _ => {}
            }

            let is_def_marker = match token_val {
                PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefNative
                | PolyglotToken::DefQueue | PolyglotToken::DefError | PolyglotToken::DefPermission
                | PolyglotToken::DefCollector | PolyglotToken::DefConstructor | PolyglotToken::DefComment => true,
                _ => false,
            };

            if is_def_marker {
                // PGE01053: Definition Scope
                if current_scope != 0 && !matches!(token_val, PolyglotToken::DefComment) {
                    let marker_char = match token_val {
                        PolyglotToken::DefPackage => "@",
                        PolyglotToken::DefData => "#",
                        PolyglotToken::DefPipeline => "-",
                        PolyglotToken::DefTrigger => "T",
                        PolyglotToken::DefWrapper => "W",
                        PolyglotToken::DefNative => "N",
                        PolyglotToken::DefQueue => "Q",
                        PolyglotToken::DefError => "!",
                        PolyglotToken::DefPermission => "_",
                        PolyglotToken::DefCollector => "*",
                        PolyglotToken::DefConstructor => "$",
                        _ => "?",
                    };
                    
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01053".to_string(),
                        name: "Definition Scope Violation".to_string(),
                        message: "Definition markers must be root level objects (Scope 0).".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some(format!("Definition markers like `{{{}}}` denote root-level objects and cannot be indented. To define a new object, remove all indentation. If you intended to execute an action or perform IO instead, use the `[{}]` (Action) or `({})` (IO) markers.", marker_char, marker_char, marker_char)),
                    });
                }

                // PGE01049: Invalid Def Target
                if i + 1 < ctx.tokens.len() {
                    let target_token = &ctx.tokens[i+1].value;
                    
                    if !(matches!(token_val, PolyglotToken::DefComment) && matches!(target_token, PolyglotToken::TokSpace)) {
                        let is_valid_target = match (token_val, target_token) {
                            (PolyglotToken::DefPackage, PolyglotToken::Registry(_)) => true,
                            (PolyglotToken::DefPackage, PolyglotToken::PackageName(_)) => true,
                            (PolyglotToken::DefPackage, PolyglotToken::Package(_)) => true,
                            (PolyglotToken::DefData, PolyglotToken::Data(_)) => true,
                            (PolyglotToken::DefPipeline, PolyglotToken::Pipeline(_)) => true,
                            (PolyglotToken::DefTrigger, PolyglotToken::Trigger(_)) => true,
                            (PolyglotToken::DefTrigger, PolyglotToken::InlineInstruction(_)) => true,
                            (PolyglotToken::DefWrapper, PolyglotToken::Wrapper(_)) => true,
                            (PolyglotToken::DefQueue, PolyglotToken::QueueConfig(_)) => true,
                            (PolyglotToken::DefError, PolyglotToken::Error(_)) => true,
                            (PolyglotToken::DefPermission, PolyglotToken::Data(_)) => true,
                            (PolyglotToken::DefCollector, PolyglotToken::Collector(_)) => true,
                            (PolyglotToken::DefConstructor, PolyglotToken::Constructor(_)) => true,
                            (PolyglotToken::DefComment, PolyglotToken::CommentText(_)) => true,
                            _ => false,
                        };

                        if !is_valid_target {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01049".to_string(),
                                name: "Invalid Definition Target".to_string(),
                                message: format!("The definition marker is followed by an invalid or unexpected token: {:?}", target_token),
                                line: ctx.tokens[i+1].line,
                                col: ctx.tokens[i+1].col,
                                snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                help: Some(get_def_target_help(token_val, target_token)),
                            });
                        }
                    }
                }
            }
        }
    }
}
