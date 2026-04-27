use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::AnalysisContext;
use super::Rule;

pub struct CircularPipelineRule;

impl Rule for CircularPipelineRule {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        let cycles = crate::compiler::algorithms::cycle_detection::detect_cycles(ctx.tokens);
        if !cycles.is_empty() {
            for cycle in cycles {
                let cycle_str = cycle.join(" -> ");
                let err_line = if !ctx.tokens.is_empty() { ctx.tokens[0].line } else { 1 };
                report.add_error(ValidationError {
                    context_snippets: vec![],
                    code: "PGE09013".to_string(),
                    name: "Circular Pipeline Call".to_string(),
                    message: format!("Circular pipeline call detected: {} — Polyglot does not support recursion", cycle_str),
                    line: err_line,
                    col: 1,
                    snippet: None,
                    help: Some("Remove the circular dependency. Polyglot pipelines must form a Directed Acyclic Graph (DAG).".to_string()),
                });
            }
        }
    }
}
