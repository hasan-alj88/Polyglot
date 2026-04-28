use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct IOSemanticsAlgorithm;

impl Rule for IOSemanticsAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let token_val = &spanned_token.value;
            let line = spanned_token.line;
            let col = spanned_token.col;

            let is_io_marker = match token_val {
                PolyglotToken::PipelineIO | PolyglotToken::DataInput | PolyglotToken::ExpanderIO 
                | PolyglotToken::CollectorIO | PolyglotToken::ContinueIOLine | PolyglotToken::InputParameterProperty 
                | PolyglotToken::OutputParameterProperty | PolyglotToken::IoParamOutFallback 
                | PolyglotToken::IoParamInFallback | PolyglotToken::IoComment => true,
                _ => false,
            };

            if is_io_marker {
                // PGE01055: IO Context Mismatch
                let context_stack = &ctx.token_contexts[i];
                if let Some(context) = context_stack.last() {
                    let ctx_type = match &context.0 {
                        PolyglotToken::DefData | PolyglotToken::DefQueue | PolyglotToken::ActionDataLoad => "Data",
                        PolyglotToken::DefCollector | PolyglotToken::ActionCollector => "Collector",
                        _ => "Pipeline",
                    };

                    let io_type = match token_val {
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
                        if ctx_line > 0 && ctx_line < line {
                            if let Some(ctx_text) = get_snippet(ctx_line, ctx.lines) {
                                context_snippets.push((ctx_line, ctx_text));
                            }
                        }

                        report.add_error(ValidationError {
                            context_snippets,
                            code: "PGE01055".to_string(),
                            name: "IO Marker Context Mismatch".to_string(),
                            message: format!("{} IO marker used inside a {} context.", io_type, ctx_type),
                            line, col, snippet: get_snippet(line, ctx.lines),
                            help: Some(help_msg.to_string()),
                        });
                    }
                }

                // PGE01050 & PGE01061: Target validation
                if i + 1 < ctx.tokens.len() {
                    let target_token = &ctx.tokens[i+1].value;
                    
                    if !(matches!(token_val, PolyglotToken::IoComment) && matches!(target_token, PolyglotToken::TokSpace)) {
                        let is_valid_target = match target_token {
                            PolyglotToken::Variable(_) | PolyglotToken::Pipeline(_) | PolyglotToken::Data(_)
                            | PolyglotToken::Registry(_) | PolyglotToken::PackageName(_) | PolyglotToken::Package(_)
                            | PolyglotToken::Trigger(_) | PolyglotToken::InlineInstruction(_) | PolyglotToken::Wrapper(_)
                            | PolyglotToken::QueueConfig(_) | PolyglotToken::Error(_) | PolyglotToken::Collector(_)
                            | PolyglotToken::Constructor(_) | PolyglotToken::ActionDataAccessFixed | PolyglotToken::ActionDataAccessFlex
                            | PolyglotToken::ActionDataLoad | PolyglotToken::PullFrom | PolyglotToken::PushInto
                            | PolyglotToken::DefaultPullFrom | PolyglotToken::DefaultPushInto | PolyglotToken::FallBackPullFrom | PolyglotToken::FallBackPushInto
                            | PolyglotToken::DataType(_) | PolyglotToken::InputParameter(_)
                            | PolyglotToken::OutputParameter(_) | PolyglotToken::TerminalData(_) | PolyglotToken::FixedSubField(_)
                            | PolyglotToken::FlexibleSubField(_) | PolyglotToken::CommentText(_) => true,
                            _ => false,
                        };

                        if !is_valid_target {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01050".to_string(),
                                name: "Invalid IO Target".to_string(),
                                message: format!("The IO marker is followed by an invalid or unexpected token: {:?}", target_token),
                                line: ctx.tokens[i+1].line,
                                col: ctx.tokens[i+1].col,
                                snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                help: Some("Ensure the IO marker points to a valid object.".to_string()),
                            });
                        }
                    }

                    // PGE01061
                    if let PolyglotToken::InputParameter(_) = target_token {
                        if !matches!(token_val, PolyglotToken::InputParameterProperty | PolyglotToken::DataInput) {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01061".to_string(),
                                name: "IO Parameter Scope Mismatch".to_string(),
                                message: format!("Input parameter property `<` must be prefixed with `(<)` IO marker, not generic `(-)` or others."),
                                line, col, snippet: get_snippet(line, ctx.lines),
                                help: Some("Input parameters must be prefixed with the `(<)` IO marker. Using the generic `(-)` is a compile error.".to_string()),
                            });
                        }
                    } else if let PolyglotToken::OutputParameter(_) = target_token {
                        if !matches!(token_val, PolyglotToken::OutputParameterProperty) {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01061".to_string(),
                                name: "IO Parameter Scope Mismatch".to_string(),
                                message: format!("Output parameter property `>` must be prefixed with `(>)` IO marker, not generic `(-)` or others."),
                                line, col, snippet: get_snippet(line, ctx.lines),
                                help: Some("Output parameters must be prefixed with the `(>)` IO marker. Using the generic `(-)` is a compile error.".to_string()),
                            });
                        }
                    }
                }
            }

            // Operator validations (PGE01051, PGE01052)
            let is_operator = match token_val {
                PolyglotToken::PullFrom | PolyglotToken::PushInto | PolyglotToken::DefaultPullFrom
                | PolyglotToken::DefaultPushInto | PolyglotToken::FallBackPullFrom | PolyglotToken::FallBackPushInto
                | PolyglotToken::IsItEqual | PolyglotToken::IsItNotEqual | PolyglotToken::IsItGreaterThan
                | PolyglotToken::IsItNotGreaterThan | PolyglotToken::IsItLessThan | PolyglotToken::IsItNotLessThan
                | PolyglotToken::IsItOtherwise | PolyglotToken::IsItInRangeInclusiveFrom | PolyglotToken::IsItInRangeExclusiveFrom
                | PolyglotToken::IsItInRangeInclusiveTo | PolyglotToken::IsItInRangeExclusiveTo | PolyglotToken::RangeSeparator => true,
                _ => false,
            };

            if is_operator {
                // PGE01051: Missing Operator Target
                if i + 1 >= ctx.tokens.len() || matches!(ctx.tokens[i+1].value, PolyglotToken::TokNewline) {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01051".to_string(),
                        name: "Missing Operator Target".to_string(),
                        message: "The data flow operator has no target on its right side.".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("Operators like `<<` or `>>` must point to a target object (e.g., a variable `$var` or property).".to_string()),
                    });
                } else {
                    // PGE01052: Invalid Operator Target
                    let target_token = &ctx.tokens[i+1].value;
                    let is_fundamentally_invalid = match target_token {
                        PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                        | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefNative
                        | PolyglotToken::DefQueue | PolyglotToken::DefError | PolyglotToken::DefPermission
                        | PolyglotToken::DefCollector | PolyglotToken::DefConstructor | PolyglotToken::ActionRegistry
                        | PolyglotToken::ActionExecSeq | PolyglotToken::ActionExecPar | PolyglotToken::PipelineIO
                        | PolyglotToken::DataInput | PolyglotToken::Scope(_) | PolyglotToken::MisplacedMarker(_) => true,
                        _ => false,
                    };

                    if is_fundamentally_invalid {
                        report.add_error(ValidationError {
                            context_snippets: vec![],
                            code: "PGE01052".to_string(),
                            name: "Invalid Operator Target".to_string(),
                            message: format!("The operator is followed by an inherently invalid token: {:?}", target_token),
                            line: ctx.tokens[i+1].line,
                            col: ctx.tokens[i+1].col,
                            snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                            help: Some("You cannot assign to or pull from a structural marker (like `{-}`, `[-]`, `(-)`). Data flow operators must target variables, data definitions, or pipelines.".to_string()),
                        });
                    }
                }
            }
        }
    }
}
