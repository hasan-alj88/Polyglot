use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct UnrecognizedDefMarkerRule;

impl Rule for UnrecognizedDefMarkerRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::InvalidDefinitionMarker(s) = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01044".to_string(),
                    name: "Unrecognized Definition Marker".to_string(),
                    message: format!("Unknown character found inside definition curly braces: `{}`", s),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Valid definition markers include `{@}`, `{#}`, `{-}`, `{T}`, etc. Check the Polyglot specification for valid markers.".to_string())
                });
            }
        }
    }
}
