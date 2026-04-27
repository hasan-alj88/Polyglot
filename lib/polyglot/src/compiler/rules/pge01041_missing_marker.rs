use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct MissingMarkerRule;

impl Rule for MissingMarkerRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::MissingMarker = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01041".to_string(),
                    name: "Missing Structural Marker".to_string(),
                    message: "Line lacks a valid structural starting bracket.".to_string(),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Ensure the line starts with a valid structural marker like `[-]`, `{-}`, or `[ ]`.".to_string())
                });
            }
        }
    }
}
