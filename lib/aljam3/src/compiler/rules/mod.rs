use crate::compiler::error::ValidationReport;
use crate::compiler::utils::AnalysisContext;

pub mod algorithms;

pub trait Rule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport);
}

pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Algorithms framework
        Box::new(algorithms::PipelineAlgorithms),
    ]
}
