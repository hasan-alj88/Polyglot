use crate::compiler::error::ValidationReport;
use crate::compiler::utils::AnalysisContext;

pub mod file_structure;
pub mod invalid_tokens;
pub mod definition_semantics;
pub mod io_semantics;
pub mod pipeline_semantics;
pub mod queue_semantics;
pub mod algorithms;
pub mod variable_state;

pub trait Rule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport);
}

pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Algorithm 1.1: File Structure checks
        Box::new(file_structure::FileStructureAlgorithm),

        // Algorithm 1.2: Invalid Tokens checks
        Box::new(invalid_tokens::InvalidTokensAlgorithm),

        // Algorithm 1.3: Definition Semantics checks
        Box::new(definition_semantics::DefinitionSemanticsAlgorithm),

        // Algorithm 2: IO Semantics checks
        Box::new(io_semantics::IOSemanticsAlgorithm),

        // Algorithm 3: Pipeline Semantics checks
        Box::new(pipeline_semantics::PipelineSemanticsAlgorithm),

        // Algorithm 3.5: Queue Semantics checks
        Box::new(queue_semantics::QueueSemanticsAlgorithm),

        // Algorithm 4: Graph and Cycle Algorithms
        Box::new(algorithms::PipelineAlgorithms),

        // Algorithm 5: Variable State Validation
        Box::new(variable_state::VariableStateAlgorithm),
    ]
}
