use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct InvalidOperatorTargetRule;

impl Rule for InvalidOperatorTargetRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let is_operator = match spanned_token.value {
                PolyglotToken::PullFrom | PolyglotToken::PushInto | PolyglotToken::DefaultPullFrom 
                | PolyglotToken::DefaultPushInto | PolyglotToken::FallBackPullFrom | PolyglotToken::FallBackPushInto => true,
                _ => false,
            };

            if is_operator {
                if i + 1 >= ctx.tokens.len() {
                    continue; // Handled by PGE01051
                }

                let target_token = &ctx.tokens[i+1].value;

                let is_fundamentally_invalid = match target_token {
                    // You cannot assign TO or pull FROM a raw marker or structural token.
                    PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                    | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefQueue
                    | PolyglotToken::DefError | PolyglotToken::DefPermission | PolyglotToken::DefCollector
                    | PolyglotToken::DefConstructor | PolyglotToken::ActionRegistry | PolyglotToken::ActionExecSeq
                    | PolyglotToken::ActionExecPar | PolyglotToken::PipelineIO | PolyglotToken::DataInput 
                    | PolyglotToken::Scope(_) | PolyglotToken::MisplacedMarker(_) => true,
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
