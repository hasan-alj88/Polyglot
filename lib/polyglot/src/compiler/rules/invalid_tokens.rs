use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct InvalidTokensAlgorithm;

impl Rule for InvalidTokensAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens.iter() {
            let token_val = &spanned_token.value;
            let line = spanned_token.line;
            let col = spanned_token.col;

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
        }
    }
}
