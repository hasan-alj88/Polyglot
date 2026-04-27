use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct UnrecognizedActionMarkerRule;

impl Rule for UnrecognizedActionMarkerRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::InvalidActionMarker(s) = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01045".to_string(),
                    name: "Unrecognized Action Marker".to_string(),
                    message: format!("Unknown character found inside action square brackets: `{}`", s),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Valid action markers include `[-]`, `[=]`, `[@]`, `[T]`, etc. Check the Polyglot specification.".to_string())
                });
            }
        }
    }
}
