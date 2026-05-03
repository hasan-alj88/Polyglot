use crate::compiler::error::ValidationReport;
use crate::compiler::utils::AnalysisContext;

pub mod algorithms;

pub trait Rule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport);
}

pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Registered Algorithms
        Box::new(algorithms::MissingTokenDetector),
    ]
}
