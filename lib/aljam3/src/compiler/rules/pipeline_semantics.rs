use crate::lexer::token::Aljam3Token;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;
use std::collections::{HashMap, HashSet};

pub struct PipelineSemanticsAlgorithm;

struct PipelineIOContract {
    inputs: HashSet<String>,
    outputs: HashSet<String>,
}

impl Rule for PipelineSemanticsAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        // Pass 1: Collect all defined pipelines and wrappers and their IO contracts
        let mut defined_pipelines: HashMap<String, PipelineIOContract> = HashMap::new();
        let mut defined_wrappers: HashMap<String, PipelineIOContract> = HashMap::new();

        let mut i = 0;
        while i < ctx.tokens.len() {
            let token_val = &ctx.tokens[i].value;
            let mut is_pipeline = false;
            let mut is_wrapper = false;
            let mut def_name = String::new();

            if let Aljam3Token::DefPipeline = token_val {
                if i + 1 < ctx.tokens.len() {
                    if let Aljam3Token::Pipeline(name) = &ctx.tokens[i+1].value {
                        is_pipeline = true;
                        def_name = name.clone();
                    }
                }
            } else if let Aljam3Token::DefWrapper = token_val {
                if i + 1 < ctx.tokens.len() {
                    if let Aljam3Token::Wrapper(name) = &ctx.tokens[i+1].value {
                        is_wrapper = true;
                        def_name = name.clone();
                    }
                }
            }

            if is_pipeline || is_wrapper {
                let mut contract = PipelineIOContract {
                    inputs: HashSet::new(),
                    outputs: HashSet::new(),
                };

                // Scan child IO tokens at Scope 1
                let mut j = i + 1;
                while j < ctx.tokens.len() {
                    let child_token = &ctx.tokens[j];
                    if let Aljam3Token::Scope(s) = child_token.value {
                        if s == 0 {
                            break;
                        }
                        if s == 1 {
                            if j + 1 < ctx.tokens.len() {
                                if matches!(ctx.tokens[j+1].value, Aljam3Token::PipelineIO) {
                                    let mut k = j + 2;
                                    while k < ctx.tokens.len() && !matches!(ctx.tokens[k].value, Aljam3Token::Scope(_)) {
                                        match &ctx.tokens[k].value {
                                            Aljam3Token::InputParameter(name) => {
                                                if !contract.inputs.insert(name.clone()) {
                                                    report.add_error(ValidationError {
                                                        context_snippets: vec![],
                                                        code: "PGE01011".to_string(),
                                                        name: "Duplicate IO Parameter Name".to_string(),
                                                        message: format!("Duplicate input parameter `<{}` declared.", name),
                                                        line: ctx.tokens[k].line,
                                                        col: ctx.tokens[k].col,
                                                        snippet: get_snippet(ctx.tokens[k].line, ctx.lines),
                                                        help: Some("Each input `<name` must be uniquely named within the pipeline IO section.".to_string())
                                                    });
                                                }
                                            }
                                            Aljam3Token::OutputParameter(name) => {
                                                if !contract.outputs.insert(name.clone()) {
                                                    report.add_error(ValidationError {
                                                        context_snippets: vec![],
                                                        code: "PGE01011".to_string(),
                                                        name: "Duplicate IO Parameter Name".to_string(),
                                                        message: format!("Duplicate output parameter `>{}` declared.", name),
                                                        line: ctx.tokens[k].line,
                                                        col: ctx.tokens[k].col,
                                                        snippet: get_snippet(ctx.tokens[k].line, ctx.lines),
                                                        help: Some("Each output `>name` must be uniquely named within the pipeline IO section.".to_string())
                                                    });
                                                }
                                            }
                                            _ => {}
                                        }
                                        k += 1;
                                    }
                                }
                            }
                        }
                    }
                    j += 1;
                }
                
                if is_pipeline {
                    defined_pipelines.insert(def_name, contract);
                } else {
                    defined_wrappers.insert(def_name, contract);
                }
            }
            i += 1;
        }

        // Pass 2: Block validations
        let mut current_scope = 0;
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let token_val = &spanned_token.value;
            let line = spanned_token.line;
            let col = spanned_token.col;

            if let Aljam3Token::Scope(s) = token_val {
                current_scope = *s;
            }

            // Wrapper validation (PGE01008 & PGE01009)
            if let Aljam3Token::ActionWrapper = token_val {
                if i + 1 < ctx.tokens.len() {
                    let mut target_name = String::new();
                    let mut is_valid_syntax = false;
                    
                    if let Aljam3Token::Wrapper(name) = &ctx.tokens[i+1].value {
                        target_name = name.clone();
                        is_valid_syntax = true;
                    }

                    if is_valid_syntax {
                        let is_aj3lib = target_name.starts_with("W.");
                        
                        // TODO: Expand this to check other packages/files
                        if !is_aj3lib && !defined_wrappers.contains_key(&target_name) {
                            if defined_pipelines.contains_key(&target_name) {
                                report.add_error(ValidationError {
                                    context_snippets: vec![],
                                    code: "PGE01008".to_string(),
                                    name: "Wrapper Must Reference Wrapper Definition".to_string(),
                                    message: format!("Wrapper call `[W]` references `{}`, which is a pipeline `{{-}}`, not a wrapper.", target_name),
                                    line: ctx.tokens[i+1].line,
                                    col: ctx.tokens[i+1].col,
                                    snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                    help: Some("`[W]` must reference a wrapper defined with `{W}`.".to_string()),
                                });
                            } else {
                                report.add_error(ValidationError {
                                    context_snippets: vec![],
                                    code: "PGE01008".to_string(),
                                    name: "Wrapper Must Reference Wrapper Definition".to_string(),
                                    message: format!("Wrapper `{}` is not defined in the current package.", target_name),
                                    line: ctx.tokens[i+1].line,
                                    col: ctx.tokens[i+1].col,
                                    snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                    help: Some("Ensure the wrapper is defined with `{W}`.".to_string()),
                                });
                            }
                        } else if !is_aj3lib {
                            let contract = defined_wrappers.get(&target_name).unwrap();
                            validate_io_contract(ctx, i, current_scope, contract, report, "PGE01009", "Wrapper IO Name Mismatch");
                        }
                    }
                }
            }

            // Pipeline Execution validation (PGE01060 & PGE01010)
            if matches!(token_val, Aljam3Token::ActionExecSeq | Aljam3Token::ActionExecPar | Aljam3Token::ActionExecBg | Aljam3Token::ActionCondSwitch) {
                if i + 1 < ctx.tokens.len() {
                    if let Aljam3Token::Pipeline(target) = &ctx.tokens[i+1].value {
                        let is_aj3lib = target.starts_with("T.") || target.starts_with("Q.") || target.starts_with("W.") || target.starts_with("Status.") || target.starts_with("File.") || target.starts_with("Do.");
                        
                        // TODO: Expand this to check other packages/files
                        if !is_aj3lib && !defined_pipelines.contains_key(target) {
                            if defined_wrappers.contains_key(target) {
                                report.add_error(ValidationError {
                                    context_snippets: vec![],
                                    code: "PGE01060".to_string(),
                                    name: "Unresolved Pipeline Reference".to_string(),
                                    message: format!("Execution action references `{}`, which is a wrapper `{{W}}`, not a pipeline `{{-}}`.", target),
                                    line: ctx.tokens[i+1].line,
                                    col: ctx.tokens[i+1].col,
                                    snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                    help: Some("Execution markers like `[-]` must reference a pipeline defined with `{-}`.".to_string()),
                                });
                            } else {
                                report.add_error(ValidationError {
                                    context_snippets: vec![],
                                    code: "PGE01060".to_string(),
                                    name: "Unresolved Pipeline Reference".to_string(),
                                    message: format!("The pipeline `{}` is not defined in the current package.", target),
                                    line: ctx.tokens[i+1].line,
                                    col: ctx.tokens[i+1].col,
                                    snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                    help: Some("Ensure the pipeline is defined. External imports must be explicitly prefixed if not in aj3lib.".to_string()),
                                });
                            }
                        } else if !is_aj3lib {
                            let contract = defined_pipelines.get(target).unwrap();
                            validate_io_contract(ctx, i, current_scope, contract, report, "PGE01010", "Pipeline IO Name Mismatch");
                        }
                    }
                }
            }

            // PGE01056: Invalid Data Field
            if matches!(token_val, Aljam3Token::DataInput | Aljam3Token::InputParameterProperty) {
                let context_stack = &ctx.token_contexts[i];
                if let Some(context) = context_stack.last() {
                    let ctx_line = context.1;
                    let mut is_data_def = false;
                    for j in (0..i).rev() {
                        if ctx.tokens[j].line == ctx_line {
                            if matches!(ctx.tokens[j].value, Aljam3Token::DefData | Aljam3Token::DefQueue) {
                                is_data_def = true;
                            }
                            break;
                        }
                    }
                    if is_data_def {
                        report.add_error(ValidationError {
                            context_snippets: vec![],
                            code: "PGE01056".to_string(),
                            name: "Invalid Data Field Definition".to_string(),
                            message: "Data field defined using IO marker `(#)` or `<` instead of `[#] .field_name#Type`.".to_string(),
                            line, col, snippet: get_snippet(line, ctx.lines),
                            help: Some("Use `[#] .field_name` to define fields inside a `{#}` Data context.".to_string()),
                        });
                    }
                }
            }

            // Pipeline Body validation
            if let Aljam3Token::DefPipeline = token_val {
                let mut has_io = false;
                let mut has_trigger = false;
                let mut has_queue = false;
                let mut has_wrapper = false;
                let mut has_setup = false;
                let mut has_teardown = false;
                let mut has_execution = false;
                
                let mut order_state = 0;

                let mut j = i + 1;
                while j < ctx.tokens.len() {
                    let child_token = &ctx.tokens[j];
                    if let Aljam3Token::Scope(s) = child_token.value {
                        if s == 0 {
                            break;
                        }
                        if s == 1 {
                            if j + 1 < ctx.tokens.len() {
                                let action_token = &ctx.tokens[j+1].value;
                                
                                match action_token {
                                    Aljam3Token::PipelineIO | Aljam3Token::DataInput | Aljam3Token::ExpanderIO 
                                    | Aljam3Token::CollectorIO | Aljam3Token::InputParameterProperty 
                                    | Aljam3Token::OutputParameterProperty => {
                                        has_io = true;
                                        if order_state > 0 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "IO parameter declaration must appear before the Trigger `[T]`.".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("The required marker order is IO -> `[T]` -> `[Q]` -> `[W]` or `[\\]` -> execution actions -> `[/]`.".to_string()),
                                            });
                                        }
                                    }
                                    Aljam3Token::ActionTrigger => {
                                        has_trigger = true;
                                        if order_state > 0 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "Trigger `[T]` must appear after IO parameters but before Queue `[Q]`.".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("The required marker order is IO -> `[T]` -> `[Q]` -> `[W]` or `[\\]` -> execution actions -> `[/]`.".to_string()),
                                            });
                                        } else {
                                            order_state = 1;
                                        }
                                    }
                                    Aljam3Token::ActionQueue => {
                                        has_queue = true;
                                        if order_state > 1 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "Queue `[Q]` must appear after Trigger `[T]` but before Wrapper `[W]` or Setup `[\\]`.".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("The required marker order is IO -> `[T]` -> `[Q]` -> `[W]` or `[\\]` -> execution actions -> `[/]`.".to_string()),
                                            });
                                        } else {
                                            order_state = 2;
                                        }
                                    }
                                    Aljam3Token::ActionWrapper => {
                                        has_wrapper = true;
                                        if order_state > 2 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "Wrapper `[W]` must appear after Queue `[Q]` and before execution actions.".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("The required marker order is IO -> `[T]` -> `[Q]` -> `[W]` or `[\\]` -> execution actions -> `[/]`.".to_string()),
                                            });
                                        } else {
                                            order_state = 3;
                                        }
                                    }
                                    Aljam3Token::ActionScopeIn => {
                                        has_setup = true;
                                        if order_state > 2 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "Setup `[\\]` must appear after Queue `[Q]` and before execution actions.".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("The required marker order is IO -> `[T]` -> `[Q]` -> `[W]` or `[\\]` -> execution actions -> `[/]`.".to_string()),
                                            });
                                        } else {
                                            order_state = 3;
                                        }
                                    }
                                    Aljam3Token::ActionScopeOut => {
                                        has_teardown = true;
                                        if order_state < 4 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "Teardown `[/]` must appear after execution actions.".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("The required marker order is IO -> `[T]` -> `[Q]` -> `[W]` or `[\\]` -> execution actions -> `[/]`.".to_string()),
                                            });
                                        }
                                        order_state = 5;
                                    }
                                    tok if is_execution_action(tok) => {
                                        has_execution = true;
                                        if order_state < 2 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "Execution actions must appear after the setup markers `[T]`, `[Q]`, and `[W]` (or `[\\]`).".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("Ensure the pipeline header has `[T]`, `[Q]`, and `[W]` declared before starting execution.".to_string()),
                                            });
                                            order_state = 4;
                                        } else if order_state == 5 {
                                            report.add_error(ValidationError {
                                                context_snippets: vec![],
                                                code: "PGE01002".to_string(),
                                                name: "Pipeline Section Misordering".to_string(),
                                                message: "Execution actions cannot appear after Teardown `[/]`.".to_string(),
                                                line: child_token.line,
                                                col: child_token.col,
                                                snippet: get_snippet(child_token.line, ctx.lines),
                                                help: Some("The required marker order is IO -> `[T]` -> `[Q]` -> `[W]` or `[\\]` -> execution actions -> `[/]`.".to_string()),
                                            });
                                        } else {
                                            order_state = 4;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    j += 1;
                }

                if !has_io {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01003".to_string(),
                        name: "Mandatory IO".to_string(),
                        message: "Pipeline lacks mandatory IO declarations.".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("You must declare inputs and outputs before the `[T]` trigger. If the pipeline has no input, use `(-) <#None`. If it has no output, use `(-) >#None`.".to_string()),
                    });
                }
                if !has_trigger {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01004".to_string(),
                        name: "Mandatory Trigger".to_string(),
                        message: "Pipeline lacks a mandatory Trigger `[T]` block.".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("All pipelines must define a Trigger `[T]`. To disable automatic triggering, specify `[T] -T.CLI`.".to_string()),
                    });
                }
                if !has_queue {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01005".to_string(),
                        name: "Mandatory Queue Config".to_string(),
                        message: "Pipeline lacks a mandatory Queue Configuration `[Q]` block.".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("All pipelines must define a Queue Config `[Q]`. To use standard behavior, specify `[Q] -Q.Default`.".to_string()),
                    });
                }
                if !has_wrapper && !(has_setup && has_teardown) {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01006".to_string(),
                        name: "Mandatory Setup/Cleanup".to_string(),
                        message: "Pipeline lacks environmental integration.".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("You must define either a Wrapper `[W]` (e.g., `[W] -W.Aljam3`) or explicitly provide an inline Setup `[\\]` and Teardown `[/]` block pair.".to_string()),
                    });
                }
                if !has_execution {
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01062".to_string(),
                        name: "Missing Execution Body".to_string(),
                        message: "Pipeline lacks an execution body.".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some("All pipelines must contain at least one execution block (e.g. `[-]`, `[=]`, `[?]`) to perform work.".to_string()),
                    });
                }
            }
        }
    }
}

fn validate_io_contract(
    ctx: &AnalysisContext,
    start_idx: usize,
    base_scope: usize,
    contract: &PipelineIOContract,
    report: &mut ValidationReport,
    error_code: &str,
    error_name: &str
) {
    let mut j = start_idx + 1;
    let expected_scope = base_scope + 1;
    while j < ctx.tokens.len() {
        let child_token = &ctx.tokens[j];
        if let Aljam3Token::Scope(s) = child_token.value {
            if s <= base_scope {
                break; // End of block
            }
            if s == expected_scope {
                if j + 1 < ctx.tokens.len() {
                    if matches!(ctx.tokens[j+1].value, Aljam3Token::PipelineIO) {
                        let mut k = j + 2;
                        while k < ctx.tokens.len() && !matches!(ctx.tokens[k].value, Aljam3Token::Scope(_)) {
                            match &ctx.tokens[k].value {
                                Aljam3Token::InputParameter(name) => {
                                    if !contract.inputs.contains(name) {
                                        let mut inputs_vec: Vec<_> = contract.inputs.iter().map(|s| format!("<{}", s)).collect();
                                        inputs_vec.sort();
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: error_code.to_string(),
                                            name: error_name.to_string(),
                                            message: format!("Supplied input parameter `<{}` does not exist in target's IO definition.", name),
                                            line: ctx.tokens[k].line,
                                            col: ctx.tokens[k].col,
                                            snippet: get_snippet(ctx.tokens[k].line, ctx.lines),
                                            help: Some(format!("Available inputs: {:?}", inputs_vec)),
                                        });
                                    }
                                }
                                Aljam3Token::OutputParameter(name) => {
                                    if !contract.outputs.contains(name) {
                                        let mut outputs_vec: Vec<_> = contract.outputs.iter().map(|s| format!(">{}", s)).collect();
                                        outputs_vec.sort();
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: error_code.to_string(),
                                            name: error_name.to_string(),
                                            message: format!("Supplied output parameter `>{}` does not exist in target's IO definition.", name),
                                            line: ctx.tokens[k].line,
                                            col: ctx.tokens[k].col,
                                            snippet: get_snippet(ctx.tokens[k].line, ctx.lines),
                                            help: Some(format!("Available outputs: {:?}", outputs_vec)),
                                        });
                                    }
                                }
                                _ => {}
                            }
                            k += 1;
                        }
                    }
                }
            }
        }
        j += 1;
    }
}

fn is_execution_action(token: &Aljam3Token) -> bool {
    matches!(
        token,
        Aljam3Token::ActionExecSeq | Aljam3Token::ActionExecPar 
        | Aljam3Token::ActionExecBg | Aljam3Token::ActionCondSwitch 
        | Aljam3Token::ActionDataLoad | Aljam3Token::ActionTypeBind
        | Aljam3Token::ActionDataAccessFixed | Aljam3Token::ActionDataAccessFlex
        | Aljam3Token::ActionForeignCode
    )
}
