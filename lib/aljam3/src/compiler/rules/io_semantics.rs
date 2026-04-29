use crate::lexer::token::Aljam3Token;
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
                Aljam3Token::PipelineIO | Aljam3Token::DataInput | Aljam3Token::ExpanderIO 
                | Aljam3Token::CollectorIO | Aljam3Token::ContinueIOLine | Aljam3Token::InputParameterProperty 
                | Aljam3Token::OutputParameterProperty | Aljam3Token::IoParamOutFallback 
                | Aljam3Token::IoParamInFallback | Aljam3Token::IoComment => true,
                _ => false,
            };

            if is_io_marker {
                // PGE01055: IO Context Mismatch
                let context_stack = &ctx.token_contexts[i];
                if let Some(context) = context_stack.last() {
                    let ctx_type = match &context.0 {
                        Aljam3Token::DefData | Aljam3Token::DefQueue | Aljam3Token::ActionDataLoad => "Data",
                        Aljam3Token::DefCollector | Aljam3Token::ActionCollector => "Collector",
                        _ => "Pipeline",
                    };

                    let io_type = match token_val {
                        Aljam3Token::DataInput => "Data",
                        Aljam3Token::CollectorIO => "Collector",
                        Aljam3Token::PipelineIO | Aljam3Token::InputParameterProperty | Aljam3Token::OutputParameterProperty 
                        | Aljam3Token::ExpanderIO | Aljam3Token::IoParamOutFallback | Aljam3Token::IoParamInFallback => "Pipeline",
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
                    
                    if !(matches!(token_val, Aljam3Token::IoComment) && matches!(target_token, Aljam3Token::TokSpace)) {
                        let is_valid_target = match target_token {
                            Aljam3Token::Variable(_) | Aljam3Token::Pipeline(_) | Aljam3Token::Data(_)
                            | Aljam3Token::Registry(_) | Aljam3Token::PackageName(_) | Aljam3Token::Package(_)
                            | Aljam3Token::Trigger(_) | Aljam3Token::InlineInstruction(_) | Aljam3Token::Wrapper(_)
                            | Aljam3Token::QueueConfig(_) | Aljam3Token::Error(_) | Aljam3Token::Collector(_)
                            | Aljam3Token::Constructor(_) | Aljam3Token::ActionDataAccessFixed | Aljam3Token::ActionDataAccessFlex
                            | Aljam3Token::ActionDataLoad | Aljam3Token::PullFrom | Aljam3Token::PushInto
                            | Aljam3Token::DefaultPullFrom | Aljam3Token::DefaultPushInto | Aljam3Token::FallBackPullFrom | Aljam3Token::FallBackPushInto
                            | Aljam3Token::DataType(_) | Aljam3Token::InputParameter(_)
                            | Aljam3Token::OutputParameter(_) | Aljam3Token::TerminalData(_) | Aljam3Token::FixedSubField(_)
                            | Aljam3Token::FlexibleSubField(_) | Aljam3Token::CommentText(_) => true,
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
                    if let Aljam3Token::InputParameter(_) = target_token {
                        if !matches!(token_val, Aljam3Token::InputParameterProperty | Aljam3Token::DataInput) {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01061".to_string(),
                                name: "IO Parameter Scope Mismatch".to_string(),
                                message: format!("Input parameter property `<` must be prefixed with `(<)` IO marker, not generic `(-)` or others."),
                                line, col, snippet: get_snippet(line, ctx.lines),
                                help: Some("Input parameters must be prefixed with the `(<)` IO marker. Using the generic `(-)` is a compile error.".to_string()),
                            });
                        }
                    } else if let Aljam3Token::OutputParameter(_) = target_token {
                        if !matches!(token_val, Aljam3Token::OutputParameterProperty) {
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
                Aljam3Token::PullFrom | Aljam3Token::PushInto | Aljam3Token::DefaultPullFrom
                | Aljam3Token::DefaultPushInto | Aljam3Token::FallBackPullFrom | Aljam3Token::FallBackPushInto
                | Aljam3Token::IsItEqual | Aljam3Token::IsItNotEqual | Aljam3Token::IsItGreaterThan
                | Aljam3Token::IsItNotGreaterThan | Aljam3Token::IsItLessThan | Aljam3Token::IsItNotLessThan
                | Aljam3Token::IsItOtherwise | Aljam3Token::IsItInRangeInclusiveFrom | Aljam3Token::IsItInRangeExclusiveFrom
                | Aljam3Token::IsItInRangeInclusiveTo | Aljam3Token::IsItInRangeExclusiveTo | Aljam3Token::RangeSeparator => true,
                _ => false,
            };

            if is_operator {
                // PGE01051: Missing Operator Target
                if i + 1 >= ctx.tokens.len() || matches!(ctx.tokens[i+1].value, Aljam3Token::TokNewline) {
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
                        Aljam3Token::DefPackage | Aljam3Token::DefData | Aljam3Token::DefPipeline
                        | Aljam3Token::DefTrigger | Aljam3Token::DefWrapper | Aljam3Token::DefNative
                        | Aljam3Token::DefQueue | Aljam3Token::DefError | Aljam3Token::DefPermission
                        | Aljam3Token::DefCollector | Aljam3Token::DefConstructor | Aljam3Token::ActionRegistry
                        | Aljam3Token::ActionExecSeq | Aljam3Token::ActionExecPar | Aljam3Token::PipelineIO
                        | Aljam3Token::DataInput | Aljam3Token::Scope(_) | Aljam3Token::MisplacedMarker(_) => true,
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
