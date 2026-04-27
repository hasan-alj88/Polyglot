use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct UnresolvedReferenceRule;

impl Rule for UnresolvedReferenceRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        let mut defined_pipelines = std::collections::HashSet::new();
        for i in 0..ctx.tokens.len() {
            if let PolyglotToken::DefPipeline = &ctx.tokens[i].value {
                if i + 1 < ctx.tokens.len() {
                    if let PolyglotToken::Pipeline(name) = &ctx.tokens[i+1].value {
                        defined_pipelines.insert(name.clone());
                    }
                }
            }
        }

        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            if matches!(spanned_token.value, PolyglotToken::ActionExecSeq | PolyglotToken::ActionExecPar | PolyglotToken::ActionExecBg | PolyglotToken::ActionCondSwitch) {
                if i + 1 < ctx.tokens.len() {
                    if let PolyglotToken::Pipeline(target) = &ctx.tokens[i+1].value {
                        let is_pglib = target.starts_with("T.") || target.starts_with("Q.") || target.starts_with("W.") || target.starts_with("Status.");
                        if !is_pglib && !defined_pipelines.contains(target) {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01060".to_string(),
                                name: "Unresolved Pipeline Reference".to_string(),
                                message: format!("The pipeline `{}` is not defined in the current package.", target),
                                line: ctx.tokens[i+1].line,
                                col: ctx.tokens[i+1].col,
                                snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                help: Some("Ensure the pipeline is defined. External imports must be explicitly prefixed if not in pglib.".to_string()),
                            });
                        }
                    }
                }
            }
        }
    }
}
