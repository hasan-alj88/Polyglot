use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct DefinitionScopeRule;

impl Rule for DefinitionScopeRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        let mut current_scope = 0;
        for spanned_token in ctx.tokens {
            if let PolyglotToken::Scope(s) = spanned_token.value {
                current_scope = s;
            }
            
            let is_def_marker = match spanned_token.value {
                PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefNative
                | PolyglotToken::DefQueue | PolyglotToken::DefError | PolyglotToken::DefPermission
                | PolyglotToken::DefCollector | PolyglotToken::DefConstructor | PolyglotToken::DefComment => true,
                _ => false,
            };

            if is_def_marker && current_scope != 0 && !matches!(spanned_token.value, PolyglotToken::DefComment) {
                let marker_char = match spanned_token.value {
                    PolyglotToken::DefPackage => "@",
                    PolyglotToken::DefData => "#",
                    PolyglotToken::DefPipeline => "-",
                    PolyglotToken::DefTrigger => "T",
                    PolyglotToken::DefWrapper => "W",
                    PolyglotToken::DefNative => "N",
                    PolyglotToken::DefQueue => "Q",
                    PolyglotToken::DefError => "!",
                    PolyglotToken::DefPermission => "_",
                    PolyglotToken::DefCollector => "*",
                    PolyglotToken::DefConstructor => "$",
                    _ => "?",
                };
                
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01053".to_string(),
                    name: "Definition Scope Violation".to_string(),
                    message: "Definition markers must be root level objects (Scope 0).".to_string(),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some(format!("Definition markers like `{{{}}}` denote root-level objects and cannot be indented. To define a new object, remove all indentation. If you intended to execute an action or perform IO instead, use the `[{}]` (Action) or `({})` (IO) markers.", marker_char, marker_char, marker_char)),
                });
            }
        }
    }
}
