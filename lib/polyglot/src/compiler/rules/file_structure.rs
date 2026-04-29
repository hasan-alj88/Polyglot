use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct FileStructureAlgorithm;

impl Rule for FileStructureAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        // PGE01001: One Package Per File
        let mut package_defs = 0;
        let mut first_non_comment_idx = None;

        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            if matches!(spanned_token.value, PolyglotToken::DefPackage) {
                package_defs += 1;
                if package_defs > 1 {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01001".to_string(),
                        name: "One Package Per File".to_string(),
                        message: "A single Polyglot file can only contain one package definition `{@}`.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("Split the code into multiple files, each containing exactly one package definition.".to_string())
                    });
                }
            }

            if first_non_comment_idx.is_none() {
                match spanned_token.value {
                    PolyglotToken::DefComment | PolyglotToken::CommentText(_) | PolyglotToken::TokSpace => {}
                    _ => {
                        first_non_comment_idx = Some(i);
                    }
                }
            }
        }

        if package_defs == 0 {
            report.add_error(ValidationError {
                context_snippets: vec![],
                code: "PGE01001".to_string(),
                name: "One Package Per File".to_string(),
                message: "File lacks a package definition.".to_string(),
                line: 1,
                col: 1,
                snippet: get_snippet(1, ctx.lines),
                help: Some("All Polyglot files must start with a package definition `{@}`.".to_string())
            });
        } else if let Some(first_idx) = first_non_comment_idx {
            if !matches!(ctx.tokens[first_idx].value, PolyglotToken::DefPackage) {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01001".to_string(),
                    name: "One Package Per File".to_string(),
                    message: "The first statement in a Polyglot file must be a package definition `{@}`.".to_string(),
                    line: ctx.tokens[first_idx].line,
                    col: ctx.tokens[first_idx].col,
                    snippet: get_snippet(ctx.tokens[first_idx].line, ctx.lines),
                    help: Some("Ensure the file starts with `{@} PackageName`.".to_string())
                });
            }
        }
    }
}
