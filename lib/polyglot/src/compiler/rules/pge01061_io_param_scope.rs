use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct IOParamScopeRule;

impl Rule for IOParamScopeRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let is_io_marker = match spanned_token.value {
                PolyglotToken::PipelineIO | PolyglotToken::DataInput | PolyglotToken::ExpanderIO 
                | PolyglotToken::CollectorIO | PolyglotToken::ContinueIOLine | PolyglotToken::InputParameterProperty 
                | PolyglotToken::OutputParameterProperty | PolyglotToken::IoParamOutFallback 
                | PolyglotToken::IoParamInFallback | PolyglotToken::IoComment => true,
                _ => false,
            };

            if is_io_marker {
                if i + 1 < ctx.tokens.len() {
                    let target_token = &ctx.tokens[i+1].value;
                    if let PolyglotToken::InputParameter(_) = target_token {
                        if !matches!(spanned_token.value, PolyglotToken::InputParameterProperty | PolyglotToken::DataInput) {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01061".to_string(),
                                name: "IO Parameter Scope Mismatch".to_string(),
                                message: format!("Input parameter property `<` must be prefixed with `(<)` IO marker, not generic `(-)` or others."),
                                line: spanned_token.line,
                                col: spanned_token.col,
                                snippet: get_snippet(spanned_token.line, ctx.lines),
                                help: Some("Input parameters must be prefixed with the `(<)` IO marker. Using the generic `(-)` is a compile error.".to_string()),
                            });
                        }
                    } else if let PolyglotToken::OutputParameter(_) = target_token {
                        if !matches!(spanned_token.value, PolyglotToken::OutputParameterProperty) {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01061".to_string(),
                                name: "IO Parameter Scope Mismatch".to_string(),
                                message: format!("Output parameter property `>` must be prefixed with `(>)` IO marker, not generic `(-)` or others."),
                                line: spanned_token.line,
                                col: spanned_token.col,
                                snippet: get_snippet(spanned_token.line, ctx.lines),
                                help: Some("Output parameters must be prefixed with the `(>)` IO marker. Using the generic `(-)` is a compile error.".to_string()),
                            });
                        }
                    }
                }
            }
        }
    }
}
