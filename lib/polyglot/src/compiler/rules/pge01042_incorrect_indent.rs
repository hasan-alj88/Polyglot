use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct IncorrectIndentRule;

impl Rule for IncorrectIndentRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::IncorrectIndent(s) = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01042".to_string(),
                    name: "Incorrect Indentation Multiple".to_string(),
                    message: format!("Indentation must be a multiple of 3. Found `{}`.", s),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Polyglot uses strict 3-space indentation to define scope. Adjust spaces to a multiple of 3.".to_string())
                });
            }
        }
    }
}
