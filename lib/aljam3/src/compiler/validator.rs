use crate::lexer::token::{Aljam3Token, Spanned};
use crate::compiler::error::ValidationReport;
use crate::compiler::utils::AnalysisContext;

pub fn validate(tokens: &[Spanned<Aljam3Token>], script: &str, file_path: &str) -> ValidationReport {
    let mut report = ValidationReport::new(file_path.to_string());
    let lines: Vec<&str> = script.lines().collect();

    let ctx = AnalysisContext::new(tokens, &lines);

    for rule in crate::compiler::rules::get_all_rules() {
        rule.validate(&ctx, &mut report);
    }

    report
}
