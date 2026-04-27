use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct UnknownObjectRule;

impl Rule for UnknownObjectRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::UnknownPolyglotObject(s) = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01047".to_string(),
                    name: "Unknown Polyglot Object".to_string(),
                    message: format!("Identifier has an unknown prefix or invalid structure: `{}`", s),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Ensure your identifiers use the correct sigil (e.g. `$` for variables, `#` for data, `-` for pipelines).".to_string())
                });
            }
        }
    }
}
