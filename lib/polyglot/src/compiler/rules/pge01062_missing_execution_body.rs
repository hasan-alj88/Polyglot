use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct MissingExecutionBodyRule;

impl Rule for MissingExecutionBodyRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            if let PolyglotToken::DefPipeline = &spanned_token.value {
                let mut has_execution_body = false;
                let mut j = i + 1;
                while j < ctx.tokens.len() {
                    let token_val = &ctx.tokens[j].value;
                    if let PolyglotToken::Scope(s) = token_val {
                        if *s == 0 {
                            break;
                        }
                    }
                    
                    let is_execution_action = match token_val {
                        PolyglotToken::ActionExecSeq | PolyglotToken::ActionExecPar 
                        | PolyglotToken::ActionExecBg | PolyglotToken::ActionCondSwitch 
                        | PolyglotToken::ActionDataLoad | PolyglotToken::ActionTypeBind
                        | PolyglotToken::ActionDataAccessFixed | PolyglotToken::ActionDataAccessFlex
                        | PolyglotToken::ActionForeignCode => true,
                        _ => false,
                    };
                    
                    if is_execution_action {
                        has_execution_body = true;
                        break;
                    }
                    j += 1;
                }
                
                if !has_execution_body {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01062".to_string(),
                        name: "Missing Execution Body".to_string(),
                        message: "Pipeline lacks an execution body.".to_string(),
                        line: spanned_token.line,
                        col: spanned_token.col,
                        snippet: get_snippet(spanned_token.line, ctx.lines),
                        help: Some("All pipelines must contain at least one execution block (e.g. `[-]`, `[=]`, `[b]`) to perform work.".to_string()),
                    });
                }
            }
        }
    }
}
