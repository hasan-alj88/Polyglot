use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct UnrecognizedIOMarkerRule;

impl Rule for UnrecognizedIOMarkerRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::InvalidIOMarker(s) = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01046".to_string(),
                    name: "Unrecognized IO Marker".to_string(),
                    message: format!("Unknown character found inside IO parentheses: `{}`", s),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Valid IO markers include `(-)`, `(#)`, `(<)`, `(>)`, etc. Check the Polyglot specification.".to_string())
                });
            }
        }
    }
}
