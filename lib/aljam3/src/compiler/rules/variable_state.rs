use crate::lexer::token::Aljam3Token;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet};
use super::Rule;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarState {
    Declared,
    Default,
    Final,
    Failed,
    Released,
}

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub state: VarState,
    pub datatype: Option<String>,
}

pub struct VariableStateAlgorithm;

impl Rule for VariableStateAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        // Scopes: index is scope level, value is map of variables in that scope
        let mut scopes: Vec<HashMap<String, VariableInfo>> = vec![HashMap::new()];
        let mut current_scope_level = 0;

        let mut i = 0;
        while i < ctx.tokens.len() {
            let token = &ctx.tokens[i];
            let line = token.line;
            let col = token.col;

            // Handle Scope changes
            if let Aljam3Token::Scope(s) = token.value {
                // If entering a deeper scope, ensure we have enough maps
                while scopes.len() <= s {
                    scopes.push(HashMap::new());
                }
                // If returning to a shallower scope, variables in deeper scopes are Released.
                // We just clear them so they can't be accessed.
                if s < current_scope_level {
                    for level in (s + 1)..scopes.len() {
                        scopes[level].clear();
                    }
                }
                current_scope_level = s;
            }

            // Detect new variables or input parameters
            match &token.value {
                Aljam3Token::InputParameter(name) => {
                    // Input parameters are implicitly Final
                    let var_name = format!("<{}", name); // using the raw name or with prefix
                    scopes[current_scope_level].insert(var_name.clone(), VariableInfo {
                        state: VarState::Final,
                        datatype: Some("Any".to_string()), // We assume it has a type from schema
                    });
                }
                Aljam3Token::Variable(name) => {
                    let mut found = false;
                    for level in (0..=current_scope_level).rev() {
                        if scopes[level].contains_key(name) {
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        // This is a declaration. Check for datatype.
                        let mut has_datatype = false;
                        let mut datatype = None;
                        
                        // Look ahead for DataType token
                        let mut j = i + 1;
                        while j < ctx.tokens.len() {
                            match &ctx.tokens[j].value {
                                Aljam3Token::TokSpace => { j += 1; continue; }
                                Aljam3Token::DataType(dt) => {
                                    has_datatype = true;
                                    datatype = Some(dt.clone());
                                    break;
                                }
                                _ => break,
                            }
                        }

                        if !has_datatype {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE02001".to_string(),
                                name: "Missing Datatype in Declaration".to_string(),
                                message: format!("Variable `{}` is declared without a datatype.", name),
                                line, col, snippet: get_snippet(line, ctx.lines),
                                help: Some("Variables must be declared with a datatype (e.g. `$var#type`) on their first use.".to_string()),
                            });
                        }

                        // Determine if it's assigned immediately
                        // If it's a destination of an assignment, state transitions will be handled by the operator scanning
                        scopes[current_scope_level].insert(name.clone(), VariableInfo {
                            state: VarState::Declared,
                            datatype,
                        });
                    }
                }
                _ => {}
            }

            // Track Assignments and Pulls
            // We look for operators and apply transitions to their left and right operands
            if let Some((op_type, dir)) = get_operator_info(&token.value) {
                // Find left operand
                let mut left_var = None;
                let mut left_idx = None;
                for j in (0..i).rev() {
                    match &ctx.tokens[j].value {
                        Aljam3Token::TokSpace | Aljam3Token::DataType(_) => continue,
                        Aljam3Token::Variable(name) => {
                            left_var = Some(name.clone());
                            left_idx = Some(j);
                            break;
                        }
                        _ => break, // Only matching direct variables for now
                    }
                }

                // Find right operand
                let mut right_var = None;
                let mut right_idx = None;
                for j in (i + 1)..ctx.tokens.len() {
                    match &ctx.tokens[j].value {
                        Aljam3Token::TokSpace | Aljam3Token::DataType(_) => continue,
                        Aljam3Token::Variable(name) => {
                            right_var = Some(name.clone());
                            right_idx = Some(j);
                            break;
                        }
                        _ => break,
                    }
                }

                let (dest_var, dest_idx, source_var, source_idx) = match dir {
                    Direction::RightToLeft => (left_var, left_idx, right_var, right_idx),
                    Direction::LeftToRight => (right_var, right_idx, left_var, left_idx),
                };

                // Apply transitions to SOURCE (Pull)
                if let (Some(s_var), Some(s_idx)) = (source_var, source_idx) {
                    let s_line = ctx.tokens[s_idx].line;
                    let s_col = ctx.tokens[s_idx].col;
                    if let Some((_level, info)) = find_variable(&mut scopes, current_scope_level, &s_var) {
                        match info.state {
                            VarState::Declared => {
                                report.add_error(ValidationError {
                                    context_snippets: vec![],
                                    code: "PGE02002".to_string(),
                                    name: "Pull from Declared Variable".to_string(),
                                    message: format!("Cannot pull from variable `{}` which is in Declared state (unassigned).", s_var),
                                    line: s_line, col: s_col, snippet: get_snippet(s_line, ctx.lines),
                                    help: Some("Ensure the variable is assigned a value before using it.".to_string()),
                                });
                            }
                            VarState::Default => {
                                // Implicit promotion to Final
                                info.state = VarState::Final;
                            }
                            VarState::Released => {
                                report.add_error(ValidationError {
                                    context_snippets: vec![],
                                    code: "PGE02008".to_string(),
                                    name: "Access Released Variable".to_string(),
                                    message: format!("Variable `{}` is released and out of scope.", s_var),
                                    line: s_line, col: s_col, snippet: get_snippet(s_line, ctx.lines),
                                    help: Some("Variables cannot be accessed after their scope ends.".to_string()),
                                });
                            }
                            _ => {} // Final is fine
                        }
                    } else {
                        // Not found at all - maybe out of scope or not defined?
                        // If it wasn't defined, it would have been caught by the declaration logic, but since this was a pull, it might be accessing a released one.
                        report.add_error(ValidationError {
                            context_snippets: vec![],
                            code: "PGE02008".to_string(),
                            name: "Access Released Variable".to_string(),
                            message: format!("Variable `{}` is released or out of scope.", s_var),
                            line: s_line, col: s_col, snippet: get_snippet(s_line, ctx.lines),
                            help: Some("Ensure the variable is in scope.".to_string()),
                        });
                    }
                }

                // Apply transitions to DESTINATION (Push)
                if let (Some(d_var), Some(d_idx)) = (dest_var, dest_idx) {
                    let d_line = ctx.tokens[d_idx].line;
                    let d_col = ctx.tokens[d_idx].col;
                    if let Some((_level, info)) = find_variable(&mut scopes, current_scope_level, &d_var) {
                        match op_type {
                            OpType::FinalPush => {
                                match info.state {
                                    VarState::Declared | VarState::Default => {
                                        info.state = VarState::Final;
                                    }
                                    VarState::Final => {
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: "PGE02003".to_string(),
                                            name: "Push to Final Variable".to_string(),
                                            message: format!("Cannot assign to variable `{}` because it is already Final.", d_var),
                                            line: d_line, col: d_col, snippet: get_snippet(d_line, ctx.lines),
                                            help: Some("Final variables cannot be reassigned. Use `<~` or `~>` for default assignment if you need reassignment.".to_string()),
                                        });
                                    }
                                    VarState::Released => {
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: "PGE02008".to_string(),
                                            name: "Access Released Variable".to_string(),
                                            message: format!("Variable `{}` is released and out of scope.", d_var),
                                            line: d_line, col: d_col, snippet: get_snippet(d_line, ctx.lines),
                                            help: None,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            OpType::DefaultPush => {
                                match info.state {
                                    VarState::Declared => {
                                        info.state = VarState::Default;
                                    }
                                    VarState::Default => {
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: "PGE02004".to_string(),
                                            name: "Double Default Push".to_string(),
                                            message: format!("Variable `{}` received a second default assignment without a final assignment in between.", d_var),
                                            line: d_line, col: d_col, snippet: get_snippet(d_line, ctx.lines),
                                            help: Some("Default-assigned variables can only receive a final push (`<<` or `>>`) next.".to_string()),
                                        });
                                    }
                                    VarState::Final => {
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: "PGE02003".to_string(),
                                            name: "Push to Final Variable".to_string(),
                                            message: format!("Cannot assign to variable `{}` because it is already Final.", d_var),
                                            line: d_line, col: d_col, snippet: get_snippet(d_line, ctx.lines),
                                            help: None,
                                        });
                                    }
                                    VarState::Released => {
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: "PGE02008".to_string(),
                                            name: "Access Released Variable".to_string(),
                                            message: format!("Variable `{}` is released and out of scope.", d_var),
                                            line: d_line, col: d_col, snippet: get_snippet(d_line, ctx.lines),
                                            help: None,
                                        });
                                    }
                                    _ => {}
                                }
                            }
                            OpType::FallbackPush => {
                                // Fallback bypasses Failed state and promotes to Final
                                match info.state {
                                    VarState::Released => {
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: "PGE02008".to_string(),
                                            name: "Access Released Variable".to_string(),
                                            message: format!("Variable `{}` is released and out of scope.", d_var),
                                            line: d_line, col: d_col, snippet: get_snippet(d_line, ctx.lines),
                                            help: None,
                                        });
                                    }
                                    VarState::Final => {
                                        report.add_error(ValidationError {
                                            context_snippets: vec![],
                                            code: "PGE02003".to_string(),
                                            name: "Push to Final Variable".to_string(),
                                            message: format!("Cannot fallback push to variable `{}` because it is already Final.", d_var),
                                            line: d_line, col: d_col, snippet: get_snippet(d_line, ctx.lines),
                                            help: None,
                                        });
                                    }
                                    _ => {
                                        info.state = VarState::Final;
                                    }
                                }
                            }
                        }
                    } else {
                        // Already handled by PGE02008
                    }
                }
            }

            i += 1;
        }

        // Schema structure topological validation (Placeholder for Not Implemented package registry check)
        // This validates if variable data types resolve structurally based on `{#}` metadata.
        // We will perform local verification here when the schema graph is fully available.
    }
}

enum OpType {
    FinalPush,
    DefaultPush,
    FallbackPush,
}

enum Direction {
    RightToLeft,
    LeftToRight,
}

fn get_operator_info(token: &Aljam3Token) -> Option<(OpType, Direction)> {
    match token {
        Aljam3Token::PullFrom => Some((OpType::FinalPush, Direction::RightToLeft)), // <<
        Aljam3Token::PushInto => Some((OpType::FinalPush, Direction::LeftToRight)), // >>
        Aljam3Token::DefaultPullFrom => Some((OpType::DefaultPush, Direction::RightToLeft)), // <~
        Aljam3Token::DefaultPushInto => Some((OpType::DefaultPush, Direction::LeftToRight)), // ~>
        Aljam3Token::FallBackPullFrom => Some((OpType::FallbackPush, Direction::RightToLeft)), // !<
        Aljam3Token::FallBackPushInto => Some((OpType::FallbackPush, Direction::LeftToRight)), // !>
        _ => None,
    }
}

fn find_variable<'a>(scopes: &'a mut Vec<HashMap<String, VariableInfo>>, current_scope: usize, name: &str) -> Option<(usize, &'a mut VariableInfo)> {
    for level in (0..=current_scope).rev() {
        if scopes[level].contains_key(name) {
            // Safe hack to return mutable reference from the correct level
            let map = &mut scopes[level];
            return map.get_mut(name).map(|info| (level, info));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::Spanned;
    use crate::compiler::error::ValidationReport;

    fn test_context(tokens: Vec<Aljam3Token>) -> ValidationReport {
        let spanned: Vec<_> = tokens.into_iter().enumerate().map(|(i, t)| Spanned::new(t, 1, i)).collect();
        let lines = vec!["test line"];
        let ctx = AnalysisContext::new(&spanned, &lines);
        let mut report = ValidationReport::new("test.aj3".to_string());
        VariableStateAlgorithm.validate(&ctx, &mut report);
        report
    }

    #[test]
    fn test_valid_transitions() {
        let tokens = vec![
            Aljam3Token::Scope(0),
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::DataType("#int".to_string()),
            Aljam3Token::DefaultPullFrom, // <~
            Aljam3Token::Data("1".to_string()),
            
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::PullFrom, // <<
            Aljam3Token::Data("2".to_string()),
        ];
        let report = test_context(tokens);
        assert!(report.violations.is_empty());
    }

    #[test]
    fn test_pge02001_missing_datatype() {
        let tokens = vec![
            Aljam3Token::Scope(0),
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::PullFrom,
            Aljam3Token::Data("1".to_string()),
        ];
        let report = test_context(tokens);
        assert!(!report.violations.is_empty());
        assert_eq!(report.violations[0].code, "PGE02001");
    }

    #[test]
    fn test_pge02002_pull_from_declared() {
        let tokens = vec![
            Aljam3Token::Scope(0),
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::DataType("#int".to_string()),
            // Declared
            Aljam3Token::Variable("$other".to_string()),
            Aljam3Token::DataType("#int".to_string()),
            Aljam3Token::PullFrom, // <<
            Aljam3Token::Variable("$var".to_string()), // Source
        ];
        let report = test_context(tokens);
        assert!(!report.violations.is_empty());
        // $var was declared but unassigned, pulling it causes PGE02002
        assert!(report.violations.iter().any(|e| e.code == "PGE02002"));
    }

    #[test]
    fn test_pge02003_push_to_final() {
        let tokens = vec![
            Aljam3Token::Scope(0),
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::DataType("#int".to_string()),
            Aljam3Token::PullFrom, // << (Final push to $var)
            Aljam3Token::Data("1".to_string()),
            
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::PullFrom, // << (Another final push to $var)
            Aljam3Token::Data("2".to_string()),
        ];
        let report = test_context(tokens);
        assert!(!report.violations.is_empty());
        assert!(report.violations.iter().any(|e| e.code == "PGE02003"));
    }

    #[test]
    fn test_pge02004_double_default_push() {
        let tokens = vec![
            Aljam3Token::Scope(0),
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::DataType("#int".to_string()),
            Aljam3Token::DefaultPullFrom, // <~ 
            Aljam3Token::Data("1".to_string()),
            
            Aljam3Token::Variable("$var".to_string()),
            Aljam3Token::DefaultPullFrom, // <~
            Aljam3Token::Data("2".to_string()),
        ];
        let report = test_context(tokens);
        assert!(!report.violations.is_empty());
        assert!(report.violations.iter().any(|e| e.code == "PGE02004"));
    }
}
