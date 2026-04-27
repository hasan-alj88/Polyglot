use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct InvalidIOTargetRule;

impl Rule for InvalidIOTargetRule {
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
                if i + 1 >= ctx.tokens.len() {
                    continue;
                }
                
                let target_token = &ctx.tokens[i+1].value;

                let is_fundamentally_invalid = matches!(target_token, 
                    PolyglotToken::Pipeline(_) | PolyglotToken::Data(_) | PolyglotToken::Package(_) | PolyglotToken::Trigger(_)
                    | PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                    | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefNative
                    | PolyglotToken::DefQueue | PolyglotToken::DefError | PolyglotToken::DefPermission
                    | PolyglotToken::DefCollector | PolyglotToken::DefConstructor | PolyglotToken::MisplacedMarker(_)
                    | PolyglotToken::ActionExecSeq | PolyglotToken::ActionExecPar | PolyglotToken::ActionTrigger
                    | PolyglotToken::ActionWrapper | PolyglotToken::ActionQueue | PolyglotToken::ActionDataLoad
                );

                if is_fundamentally_invalid {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01050".to_string(),
                        name: "Invalid IO Target".to_string(),
                        message: format!("The IO marker is followed by an inherently invalid token: {:?}", target_token),
                        line: ctx.tokens[i+1].line,
                        col: ctx.tokens[i+1].col,
                        snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                        help: Some("IO markers (like `(-)`, `(#)`) must be followed by data parameters, variables, or data flow operators, not complete structural definitions or action calls.".to_string()),
                    });
                }
            }
        }
    }
}
