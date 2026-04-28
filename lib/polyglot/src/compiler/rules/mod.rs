use crate::compiler::error::ValidationReport;
use crate::compiler::utils::AnalysisContext;

pub mod lexical_structural;
pub mod io_semantics;
pub mod pipeline_semantics;
pub mod algorithms;

pub trait Rule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport);
}

pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Algorithm 1: Lexical and Structural checks
        Box::new(lexical_structural::LexicalStructuralAlgorithm),

        // Algorithm 2: IO Semantics checks
        Box::new(io_semantics::IOSemanticsAlgorithm),

        // Algorithm 3: Pipeline Semantics checks
        Box::new(pipeline_semantics::PipelineSemanticsAlgorithm),

        // Algorithm 4: Graph and Cycle Algorithms
        Box::new(algorithms::PipelineAlgorithms),
    ]
}
