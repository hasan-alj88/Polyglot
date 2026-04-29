use crate::lexer::token::Aljam3Token;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;
use std::collections::{HashMap, HashSet};

pub struct QueueSemanticsAlgorithm;

#[derive(Default, Clone)]
struct QueueConstraints {
    strategy: Option<String>,
    host: Option<String>,
    max_instances: Option<u32>,
    max_concurrent: Option<u32>,
    resource_tags: HashSet<String>,
    kill_propagation: Option<String>,
    max_wait_time: Option<String>,
}

impl Rule for QueueSemanticsAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        // Pass 1: Build the Queue Registry from {Q} definitions
        let mut queue_registry: HashMap<String, QueueConstraints> = HashMap::new();

        let mut i = 0;
        while i < ctx.tokens.len() {
            let token_val = &ctx.tokens[i].value;
            
            if let Aljam3Token::DefQueue = token_val {
                if i + 1 < ctx.tokens.len() {
                    if let Aljam3Token::QueueConfig(name) = &ctx.tokens[i+1].value {
                        let def_name = name.clone();
                        let mut constraints = QueueConstraints::default();
                        
                        let mut j = i + 2;
                        while j < ctx.tokens.len() {
                            let child_token = &ctx.tokens[j];
                            if let Aljam3Token::Scope(s) = child_token.value {
                                if s == 0 {
                                    break;
                                }
                                if s == 1 {
                                    // Parse fields [.] .field << value
                                    if j + 1 < ctx.tokens.len() && matches!(ctx.tokens[j+1].value, Aljam3Token::DataInput) {
                                        if j + 2 < ctx.tokens.len() {
                                            if let Aljam3Token::DataField(field_name) = &ctx.tokens[j+2].value {
                                                // Extract value assigned to the field
                                                let mut k = j + 3;
                                                let mut value = String::new();
                                                while k < ctx.tokens.len() && !matches!(ctx.tokens[k].value, Aljam3Token::Scope(_)) {
                                                    if let Aljam3Token::PushInto = ctx.tokens[k].value {
                                                        if k + 1 < ctx.tokens.len() {
                                                            match &ctx.tokens[k+1].value {
                                                                Aljam3Token::StringLiteral(v) => value = v.clone(),
                                                                Aljam3Token::Data(v) => value = v.clone(),
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                    k += 1;
                                                }
                                                
                                                match field_name.as_str() {
                                                    "strategy" => constraints.strategy = Some(value),
                                                    "host" => constraints.host = Some(value),
                                                    "maxInstances" => if let Ok(v) = value.parse::<u32>() { constraints.max_instances = Some(v) },
                                                    "maxConcurrent" => if let Ok(v) = value.parse::<u32>() { constraints.max_concurrent = Some(v) },
                                                    "killPropagation" => constraints.kill_propagation = Some(value),
                                                    "maxWaitTime" => constraints.max_wait_time = Some(value),
                                                    "resourceTags" => { constraints.resource_tags.insert(value); },
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            j += 1;
                        }
                        queue_registry.insert(def_name, constraints);
                    }
                }
            }
            i += 1;
        }

        // Pass 2: Validate Queue Assignments and Constraints within Pipelines
        let mut i = 0;
        while i < ctx.tokens.len() {
            let token_val = &ctx.tokens[i].value;
            
            if let Aljam3Token::DefPipeline = token_val {
                let mut pipeline_name = String::new();
                if i + 1 < ctx.tokens.len() {
                    if let Aljam3Token::Pipeline(name) = &ctx.tokens[i+1].value {
                        pipeline_name = name.clone();
                    }
                }
                
                // Track active queue assignments and their scopes
                // scope -> queue name
                let mut active_queues: HashMap<usize, String> = HashMap::new();
                let mut current_scope = 0;
                let mut in_q_block = false;

                let mut j = i + 1;
                while j < ctx.tokens.len() {
                    let child_token = &ctx.tokens[j];
                    let line = child_token.line;
                    let col = child_token.col;
                    
                    if let Aljam3Token::Scope(s) = child_token.value {
                        if s == 0 {
                            break; // End of pipeline
                        }
                        current_scope = s;
                        if s < 2 {
                            in_q_block = false; // Exited [Q] block (usually scope 2 nested)
                            active_queues.retain(|&scope_lvl, _| scope_lvl <= current_scope);
                        }
                    }

                    if let Aljam3Token::ActionQueue = child_token.value {
                        in_q_block = true;
                    }

                    if in_q_block {
                        // Detect queue assignment: -Q.Assign"QueueName"
                        if let Aljam3Token::Pipeline(p_name) = &child_token.value {
                            if p_name.starts_with("Q.Assign\"") {
                                let q_name = p_name.trim_start_matches("Q.Assign\"").trim_end_matches("\"").to_string();
                                active_queues.insert(current_scope, q_name);
                            }
                        }

                        // Validate queue constraints
                        if let Aljam3Token::InputParameter(param_name) = &child_token.value {
                            // Find the active queue for the current scope
                            let mut active_q = None;
                            for scope_lvl in (0..=current_scope).rev() {
                                if let Some(q) = active_queues.get(&scope_lvl) {
                                    active_q = Some(q.clone());
                                    break;
                                }
                            }

                            if let Some(q_name) = active_q {
                                if let Some(constraints) = queue_registry.get(&q_name) {
                                    // Parse the value being assigned to the parameter
                                    let mut k = j + 1;
                                    let mut local_value = String::new();
                                    while k < ctx.tokens.len() && !matches!(ctx.tokens[k].value, Aljam3Token::Scope(_)) {
                                        if let Aljam3Token::PushInto = ctx.tokens[k].value {
                                            if k + 1 < ctx.tokens.len() {
                                                match &ctx.tokens[k+1].value {
                                                    Aljam3Token::StringLiteral(v) => local_value = v.clone(),
                                                    Aljam3Token::Data(v) => local_value = v.clone(),
                                                    _ => {}
                                                }
                                            }
                                        }
                                        k += 1;
                                    }

                                    // Validate against registry
                                    match param_name.as_str() {
                                        "strategy" => {
                                            if let Some(ref reg_val) = constraints.strategy {
                                                if &local_value != reg_val {
                                                    report.add_error(ValidationError {
                                                        context_snippets: vec![],
                                                        code: "PGE01064".to_string(),
                                                        name: "Queue Strategy Override".to_string(),
                                                        message: format!("Pipeline `{}` attempts to override queue `{}` strategy to `{}` (Queue requires `{}`).", pipeline_name, q_name, local_value, reg_val),
                                                        line, col, snippet: get_snippet(line, ctx.lines),
                                                        help: Some("PGE01064: A pipeline must not attempt to alter the `.strategy` defined by its assigned queue.".to_string())
                                                    });
                                                }
                                            }
                                        },
                                        "host" => {
                                            if let Some(ref reg_val) = constraints.host {
                                                if &local_value != reg_val {
                                                    report.add_error(ValidationError {
                                                        context_snippets: vec![],
                                                        code: "PGE01065".to_string(),
                                                        name: "Queue Host Override".to_string(),
                                                        message: format!("Pipeline attempts to override queue host to `{}` (Queue requires `{}`).", local_value, reg_val),
                                                        line, col, snippet: get_snippet(line, ctx.lines),
                                                        help: Some("PGE01065: A pipeline cannot request execution on a host that differs from the queue's bound host.".to_string())
                                                    });
                                                }
                                            }
                                        },
                                        "maxInstances" => {
                                            if let Ok(local_val) = local_value.parse::<u32>() {
                                                if let Some(reg_val) = constraints.max_instances {
                                                    if local_val > reg_val {
                                                        report.add_error(ValidationError {
                                                            context_snippets: vec![],
                                                            code: "PGE01066".to_string(),
                                                            name: "Queue Max Instances Exceeded".to_string(),
                                                            message: format!("Pipeline `<maxInstances` ({}) exceeds queue `{}` limit ({}).", local_val, q_name, reg_val),
                                                            line, col, snippet: get_snippet(line, ctx.lines),
                                                            help: Some("PGE01066: A pipeline's local `<maxInstances` cannot exceed the queue's `.maxInstances`.".to_string())
                                                        });
                                                    }
                                                }
                                            }
                                        },
                                        "maxConcurrent" => {
                                            if let Ok(local_val) = local_value.parse::<u32>() {
                                                if let Some(reg_val) = constraints.max_concurrent {
                                                    if local_val > reg_val {
                                                        report.add_error(ValidationError {
                                                            context_snippets: vec![],
                                                            code: "PGE01067".to_string(),
                                                            name: "Queue Max Concurrent Exceeded".to_string(),
                                                            message: format!("Pipeline `<maxConcurrent` ({}) exceeds queue `{}` limit ({}).", local_val, q_name, reg_val),
                                                            line, col, snippet: get_snippet(line, ctx.lines),
                                                            help: Some("PGE01067: A pipeline's local `<maxConcurrent` cannot exceed the queue's `.maxConcurrent`.".to_string())
                                                        });
                                                    }
                                                }
                                            }
                                        },
                                        "killPropagation" => {
                                            if let Some(ref reg_val) = constraints.kill_propagation {
                                                if &local_value != reg_val {
                                                    report.add_error(ValidationError {
                                                        context_snippets: vec![],
                                                        code: "PGE01068".to_string(),
                                                        name: "Queue Kill Propagation Conflict".to_string(),
                                                        message: format!("Pipeline attempts to alter `.killPropagation` to `{}` (Queue requires `{}`).", local_value, reg_val),
                                                        line, col, snippet: get_snippet(line, ctx.lines),
                                                        help: Some("PGE01068: A pipeline cannot alter the `.killPropagation` behavior defined strictly by the queue.".to_string())
                                                    });
                                                }
                                            }
                                        },
                                        "maxWaitTime" => {
                                            // Simplistic string compare for maxWaitTime, actual implementation might parse durations
                                            // if required, but for PGE01069 exact matching or duration parsing is needed.
                                            // We will flag mismatch for now or parse it if we add duration utils.
                                        },
                                        "resourceTags" => {
                                            if !constraints.resource_tags.contains(&local_value) {
                                                report.add_error(ValidationError {
                                                    context_snippets: vec![],
                                                    code: "PGE01070".to_string(),
                                                    name: "Queue Resource Tag Mismatch".to_string(),
                                                    message: format!("Pipeline injects `<resourceTags` requirement `{}` that the assigned queue `{}` does not possess.", local_value, q_name),
                                                    line, col, snippet: get_snippet(line, ctx.lines),
                                                    help: Some("PGE01070: A pipeline cannot inject a `<resourceTags` requirement that the assigned queue does not possess.".to_string())
                                                });
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
    }
}



