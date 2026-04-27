use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct InvalidPatternRule;

impl Rule for InvalidPatternRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::InvalidPattern(s) = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01043".to_string(),
                    name: "Invalid Lexical Pattern".to_string(),
                    message: format!("Sequence violates basic lexical grammar: `{}`", s),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Check for syntax typos. Ensure strings are closed properly and variable sigils are valid.".to_string())
                });
            }
        }
    }
}
