use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet, get_def_target_help};
use super::Rule;

pub struct InvalidDefTargetRule;

impl Rule for InvalidDefTargetRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let is_def_marker = match spanned_token.value {
                PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefNative
                | PolyglotToken::DefQueue | PolyglotToken::DefError | PolyglotToken::DefPermission
                | PolyglotToken::DefCollector | PolyglotToken::DefConstructor | PolyglotToken::DefComment => true,
                _ => false,
            };

            if is_def_marker {
                if i + 1 >= ctx.tokens.len() {
                    continue;
                }
                
                let target_token = &ctx.tokens[i+1].value;
                
                if matches!(spanned_token.value, PolyglotToken::DefComment) && matches!(target_token, PolyglotToken::TokSpace) {
                    continue;
                }

                let is_valid_target = match (&spanned_token.value, target_token) {
                    (PolyglotToken::DefPackage, PolyglotToken::Registry(_)) => true,
                    (PolyglotToken::DefPackage, PolyglotToken::PackageName(_)) => true,
                    (PolyglotToken::DefPackage, PolyglotToken::Package(_)) => true,
                    (PolyglotToken::DefData, PolyglotToken::Data(_)) => true,
                    (PolyglotToken::DefPipeline, PolyglotToken::Pipeline(_)) => true,
                    (PolyglotToken::DefTrigger, PolyglotToken::Trigger(_)) => true,
                    (PolyglotToken::DefTrigger, PolyglotToken::InlineInstruction(_)) => true,
                    (PolyglotToken::DefWrapper, PolyglotToken::Wrapper(_)) => true,
                    (PolyglotToken::DefQueue, PolyglotToken::QueueConfig(_)) => true,
                    (PolyglotToken::DefError, PolyglotToken::Error(_)) => true,
                    (PolyglotToken::DefPermission, PolyglotToken::Data(_)) => true,
                    (PolyglotToken::DefCollector, PolyglotToken::Collector(_)) => true,
                    (PolyglotToken::DefConstructor, PolyglotToken::Constructor(_)) => true,
                    (PolyglotToken::DefComment, PolyglotToken::CommentText(_)) => true,
                    _ => false,
                };

                if !is_valid_target {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01049".to_string(),
                        name: "Invalid Definition Target".to_string(),
                        message: format!("The definition marker is followed by an invalid or unexpected token: {:?}", target_token),
                        line: ctx.tokens[i+1].line,
                        col: ctx.tokens[i+1].col,
                        snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                        help: Some(get_def_target_help(&spanned_token.value, target_token)),
                    });
                }
            }
        }
    }
}
