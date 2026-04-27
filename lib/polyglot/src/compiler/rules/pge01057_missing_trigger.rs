use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct MissingTriggerRule;

impl Rule for MissingTriggerRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            if let PolyglotToken::DefPipeline = &spanned_token.value {
                let mut has_trigger = false;
                let mut j = i + 1;
                while j < ctx.tokens.len() {
                    if let PolyglotToken::Scope(s) = ctx.tokens[j].value {
                        if s == 0 {
                            break;
                        }
                        if s == 1 {
                            if j + 1 < ctx.tokens.len() {
                                if let PolyglotToken::ActionTrigger = ctx.tokens[j+1].value {
                                    has_trigger = true;
                                }
                            }
                        }
                    }
                    j += 1;
                }
                
                if !has_trigger {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01057".to_string(),
                        name: "Missing Mandatory Trigger".to_string(),
                        message: "Pipeline lacks a mandatory Trigger `[T]` block.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("All pipelines must define a Trigger `[T]`. If this pipeline is manually invoked only, specify `[T] -T.Manual`.".to_string()),
                    });
                }
            }
        }
    }
}
