use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct InvalidDataFieldRule;

impl Rule for InvalidDataFieldRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            if matches!(spanned_token.value, PolyglotToken::DataInput | PolyglotToken::InputParameterProperty) {
                let mut in_data_ctx = false;
                let mut ctx_line = 0;
                
                let current_scope = match ctx.tokens[i].value {
                    PolyglotToken::Scope(s) => s,
                    _ => {
                        let mut s = 0;
                        for j in (0..i).rev() {
                            if let PolyglotToken::Scope(scope) = ctx.tokens[j].value {
                                s = scope;
                                break;
                            }
                        }
                        s
                    }
                };
                
                for j in (0..i).rev() {
                    if let PolyglotToken::Scope(s) = ctx.tokens[j].value {
                        if s < current_scope {
                            if j + 1 < ctx.tokens.len() {
                                if matches!(ctx.tokens[j+1].value, PolyglotToken::DefData | PolyglotToken::DefQueue) {
                                    in_data_ctx = true;
                                    ctx_line = ctx.tokens[j+1].line;
                                }
                            }
                            break;
                        }
                    }
                }
                
                if in_data_ctx {
                    let mut context_snippets = Vec::new();
                    if let Some(ctx_text) = get_snippet(ctx_line, ctx.lines) {
                        context_snippets.push((ctx_line, ctx_text));
                    }
                    report.add_error(ValidationError {
                        context_snippets,
                        code: "PGE01056".to_string(),
                        name: "Invalid Data Field Definition".to_string(),
                        message: "Data field defined using IO marker `(#)` or `<` instead of `[#] .field_name#Type`.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("Inside Data `{#}` or Queue `{Q}` contexts, define fields using the data load action `[#] .my_field#Type` instead of input parameters.".to_string()),
                    });
                }
            }
        }
    }
}
