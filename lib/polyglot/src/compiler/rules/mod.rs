use crate::compiler::error::ValidationReport;
use crate::compiler::utils::AnalysisContext;

pub mod pge01041_missing_marker;
pub mod pge01042_incorrect_indent;
pub mod pge01043_invalid_pattern;
pub mod pge01044_unrecognized_def_marker;
pub mod pge01045_unrecognized_action_marker;
pub mod pge01046_unrecognized_io_marker;
pub mod pge01047_unknown_object;
pub mod pge01049_invalid_def_target;
pub mod pge01050_invalid_io_target;
pub mod pge01051_missing_operator_target;
pub mod pge01052_invalid_operator_target;
pub mod pge01053_definition_scope;
pub mod pge01054_misplaced_marker;
pub mod pge01055_io_context_mismatch;
pub mod pge01056_invalid_data_field;
pub mod pge01057_missing_trigger;
pub mod pge01058_missing_queue;
pub mod pge01059_missing_wrapper;
pub mod pge01060_unresolved_reference;
pub mod pge01061_io_param_scope;
pub mod pge01062_missing_execution_body;
pub mod pge09013_circular_pipeline;

pub trait Rule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport);
}

pub fn get_all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Lexical & Structural
        Box::new(pge01041_missing_marker::MissingMarkerRule),
        Box::new(pge01042_incorrect_indent::IncorrectIndentRule),
        Box::new(pge01043_invalid_pattern::InvalidPatternRule),
        Box::new(pge01044_unrecognized_def_marker::UnrecognizedDefMarkerRule),
        Box::new(pge01045_unrecognized_action_marker::UnrecognizedActionMarkerRule),
        Box::new(pge01046_unrecognized_io_marker::UnrecognizedIOMarkerRule),
        Box::new(pge01047_unknown_object::UnknownObjectRule),
        Box::new(pge01054_misplaced_marker::MisplacedMarkerRule),
        Box::new(pge01053_definition_scope::DefinitionScopeRule),
        Box::new(pge01049_invalid_def_target::InvalidDefTargetRule),

        // IO Semantics
        Box::new(pge01055_io_context_mismatch::IOContextMismatchRule),
        Box::new(pge01061_io_param_scope::IOParamScopeRule),
        Box::new(pge01050_invalid_io_target::InvalidIOTargetRule),
        Box::new(pge01051_missing_operator_target::MissingOperatorTargetRule),
        Box::new(pge01052_invalid_operator_target::InvalidOperatorTargetRule),

        // Pipeline Semantics
        Box::new(pge01056_invalid_data_field::InvalidDataFieldRule),
        Box::new(pge01060_unresolved_reference::UnresolvedReferenceRule),
        Box::new(pge01057_missing_trigger::MissingTriggerRule),
        Box::new(pge01058_missing_queue::MissingQueueRule),
        Box::new(pge01059_missing_wrapper::MissingWrapperRule),
        Box::new(pge01062_missing_execution_body::MissingExecutionBodyRule),

        // Algorithms
        Box::new(pge09013_circular_pipeline::CircularPipelineRule),
    ]
}
