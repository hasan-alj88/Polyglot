use crate::lexer::token::PolyglotToken;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;

pub struct MisplacedMarkerRule;

impl Rule for MisplacedMarkerRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        for spanned_token in ctx.tokens {
            if let PolyglotToken::MisplacedMarker(m) = &spanned_token.value {
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE01054".to_string(),
                    name: "Misplaced Structural Marker".to_string(),
                    message: format!("Structural marker `{}` found mid-expression. Markers must be placed at the start of the line, exactly after the Scope indentation.", m),
                    line: spanned_token.line,
                    col: spanned_token.col,
                    snippet: get_snippet(spanned_token.line, ctx.lines),
                    help: Some("Move this marker to a new line, ensuring it follows the correct indentation for its intended scope.".to_string())
                });
            }
        }
    }
}
