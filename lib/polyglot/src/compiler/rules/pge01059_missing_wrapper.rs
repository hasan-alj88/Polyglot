use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct MissingWrapperRule;

impl Rule for MissingWrapperRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            if let PolyglotToken::DefPipeline = &spanned_token.value {
                let mut has_wrapper = false;
                let mut j = i + 1;
                while j < ctx.tokens.len() {
                    if let PolyglotToken::Scope(s) = ctx.tokens[j].value {
                        if s == 0 {
                            break;
                        }
                        if s == 1 {
                            if j + 1 < ctx.tokens.len() {
                                if let PolyglotToken::ActionWrapper = ctx.tokens[j+1].value {
                                    has_wrapper = true;
                                }
                            }
                        }
                    }
                    j += 1;
                }
                
                if !has_wrapper {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01059".to_string(),
                        name: "Missing Mandatory Wrapper".to_string(),
                        message: "Pipeline lacks a mandatory Wrapper `[W]` block.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("All pipelines must define a Wrapper `[W]`. To use the default setup/cleanup, specify `[W] -W.Polyglot`.".to_string()),
                    });
                }
            }
        }
    }
}
