use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct MissingQueueRule;

impl Rule for MissingQueueRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            if let PolyglotToken::DefPipeline = &spanned_token.value {
                let mut has_queue = false;
                let mut j = i + 1;
                while j < ctx.tokens.len() {
                    if let PolyglotToken::Scope(s) = ctx.tokens[j].value {
                        if s == 0 {
                            break;
                        }
                        if s == 1 {
                            if j + 1 < ctx.tokens.len() {
                                if let PolyglotToken::ActionQueue = ctx.tokens[j+1].value {
                                    has_queue = true;
                                }
                            }
                        }
                    }
                    j += 1;
                }
                
                if !has_queue {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01058".to_string(),
                        name: "Missing Mandatory Queue Config".to_string(),
                        message: "Pipeline lacks a mandatory Queue Configuration `[Q]` block.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("All pipelines must define a Queue Config `[Q]`. To use standard behavior, specify `[Q] -Q.Default`.".to_string()),
                    });
                }
            }
        }
    }
}
