use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct MissingOperatorTargetRule;

impl Rule for MissingOperatorTargetRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let is_operator = match spanned_token.value {
                PolyglotToken::PullFrom | PolyglotToken::PushInto | PolyglotToken::DefaultPullFrom 
                | PolyglotToken::DefaultPushInto | PolyglotToken::FallBackPullFrom | PolyglotToken::FallBackPushInto => true,
                _ => false,
            };

            if is_operator {
                if i + 1 >= ctx.tokens.len() {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01051".to_string(),
                        name: "Missing Operator Target".to_string(),
                        message: "An assignment or data operator is missing a target token at the end of the line.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("Provide a valid target identifier immediately following this operator. Polyglot requires explicit targets for all data flow operations.".to_string()),
                    });
                    continue;
                }

                let target_token = &ctx.tokens[i+1].value;

                if matches!(target_token, PolyglotToken::TokNewline) {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01051".to_string(),
                        name: "Missing Operator Target".to_string(),
                        message: "An assignment or data operator is missing a target token before the newline.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("Provide a valid target identifier immediately following this operator. Polyglot requires explicit targets for all data flow operations.".to_string()),
                    });
                }
            }
        }
    }
}
