use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct IOContextMismatchRule;

impl Rule for IOContextMismatchRule {
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
                let context_stack = &ctx.token_contexts[i];
                if let Some(context) = context_stack.last() {
                    let ctx_type = match &context.0 {
                        PolyglotToken::DefData | PolyglotToken::DefQueue | PolyglotToken::ActionDataLoad => "Data",
                        PolyglotToken::DefCollector | PolyglotToken::ActionCollector => "Collector",
                        _ => "Pipeline",
                    };

                    let io_type = match spanned_token.value {
                        PolyglotToken::DataInput => "Data",
                        PolyglotToken::CollectorIO => "Collector",
                        PolyglotToken::PipelineIO | PolyglotToken::InputParameterProperty | PolyglotToken::OutputParameterProperty 
                        | PolyglotToken::ExpanderIO | PolyglotToken::IoParamOutFallback | PolyglotToken::IoParamInFallback => "Pipeline",
                        _ => "Unknown",
                    };

                    if io_type != "Unknown" && ctx_type != io_type {
                        let help_msg = match ctx_type {
                            "Data" => "Data contexts (like `{#}` or `{Q}`) require Data IO markers like `(#)`.",
                            "Collector" => "Collector contexts (like `{*}`) require Collector IO markers like `(*)`.",
                            _ => "Pipeline contexts (like `{-}` or `[-]`) require Pipeline IO markers like `(-)`, `(<)`, or `(>)`.",
                        };

                        let mut context_snippets = Vec::new();
                        let ctx_line = context.1;
                        if ctx_line > 0 && ctx_line < spanned_token.line {
                            if let Some(ctx_text) = get_snippet(ctx_line, ctx.lines) {
                                context_snippets.push((ctx_line, ctx_text));
                            }
                        }

                        report.add_error(ValidationError {
                            context_snippets,
                            code: "PGE01055".to_string(),
                            name: "IO Marker Context Mismatch".to_string(),
                            message: format!("{} IO marker used inside a {} context.", io_type, ctx_type),
                            line: spanned_token.line,
                            col: spanned_token.col,
                            snippet: get_snippet(spanned_token.line, ctx.lines),
                            help: Some(help_msg.to_string()),
                        });
                    }
                }
            }
        }
    }
}
