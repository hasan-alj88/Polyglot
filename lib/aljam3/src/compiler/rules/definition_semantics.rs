use crate::lexer::token::Aljam3Token;
use crate::compiler::error::{ValidationReport, ValidationError};
use crate::compiler::utils::{AnalysisContext, get_snippet, get_def_target_help};
use super::Rule;

pub struct DefinitionSemanticsAlgorithm;

impl Rule for DefinitionSemanticsAlgorithm {
    fn validate(&self, ctx: &AnalysisContext, report: &mut ValidationReport) {
        let mut current_scope = 0;
        
        for (i, spanned_token) in ctx.tokens.iter().enumerate() {
            let token_val = &spanned_token.value;
            let line = spanned_token.line;
            let col = spanned_token.col;

            if let Aljam3Token::Scope(s) = token_val {
                current_scope = *s;
            }

            let is_def_marker = match token_val {
                Aljam3Token::DefPackage | Aljam3Token::DefData | Aljam3Token::DefPipeline
                | Aljam3Token::DefTrigger | Aljam3Token::DefWrapper | Aljam3Token::DefNative
                | Aljam3Token::DefQueue | Aljam3Token::DefError | Aljam3Token::DefPermission
                | Aljam3Token::DefCollector | Aljam3Token::DefConstructor | Aljam3Token::DefComment => true,
                _ => false,
            };

            if is_def_marker {
                // PGE01053: Definition Scope
                if current_scope != 0 && !matches!(token_val, Aljam3Token::DefComment) {
                    let marker_char = match token_val {
                        Aljam3Token::DefPackage => "@",
                        Aljam3Token::DefData => "#",
                        Aljam3Token::DefPipeline => "-",
                        Aljam3Token::DefTrigger => "T",
                        Aljam3Token::DefWrapper => "W",
                        Aljam3Token::DefNative => "N",
                        Aljam3Token::DefQueue => "Q",
                        Aljam3Token::DefError => "!",
                        Aljam3Token::DefPermission => "_",
                        Aljam3Token::DefCollector => "*",
                        Aljam3Token::DefConstructor => "$",
                        _ => "?",
                    };
                    
                    report.add_error(ValidationError {
                        context_snippets: vec![],
                        code: "PGE01053".to_string(),
                        name: "Definition Scope Violation".to_string(),
                        message: "Definition markers must be root level objects (Scope 0).".to_string(),
                        line, col, snippet: get_snippet(line, ctx.lines),
                        help: Some(format!("Definition markers like `{{{}}}` denote root-level objects and cannot be indented. To define a new object, remove all indentation. If you intended to execute an action or perform IO instead, use the `[{}]` (Action) or `({})` (IO) markers.", marker_char, marker_char, marker_char)),
                    });
                }

                if i + 1 < ctx.tokens.len() {
                    let target_token = &ctx.tokens[i+1].value;
                    
                    // PGE01012: Queue Prefix Validation
                    if let Aljam3Token::DefQueue = token_val {
                        if let Aljam3Token::QueueConfig(name) = target_token {
                            if !name.starts_with("#Queue:") && !name.starts_with("#JobRules:") && !name.starts_with("#QueueRules:") {
                                report.add_error(ValidationError {
                                    context_snippets: vec![],
                                    code: "PGE01012".to_string(),
                                    name: "Queue Definition Must Use Required Prefix".to_string(),
                                    message: format!("Queue definition `{{Q}}` identifier must begin with `#Queue:`, `#JobRules:`, or `#QueueRules:`. Found: `{}`", name),
                                    line: ctx.tokens[i+1].line,
                                    col: ctx.tokens[i+1].col,
                                    snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                    help: Some("Prefix the queue name appropriately, e.g., `#Queue:MyQueue`.".to_string()),
                                });
                            }
                        }
                    }

                    // PGE01049: Invalid Def Target
                    if !(matches!(token_val, Aljam3Token::DefComment) && matches!(target_token, Aljam3Token::TokSpace)) {
                        let is_valid_target = match (token_val, target_token) {
                            (Aljam3Token::DefPackage, Aljam3Token::Registry(_)) => true,
                            (Aljam3Token::DefPackage, Aljam3Token::PackageName(_)) => true,
                            (Aljam3Token::DefPackage, Aljam3Token::Package(_)) => true,
                            (Aljam3Token::DefData, Aljam3Token::Data(_)) => true,
                            (Aljam3Token::DefPipeline, Aljam3Token::Pipeline(_)) => true,
                            (Aljam3Token::DefTrigger, Aljam3Token::Trigger(_)) => true,
                            (Aljam3Token::DefTrigger, Aljam3Token::InlineInstruction(_)) => true,
                            (Aljam3Token::DefWrapper, Aljam3Token::Wrapper(_)) => true,
                            (Aljam3Token::DefQueue, Aljam3Token::QueueConfig(_)) => true,
                            (Aljam3Token::DefError, Aljam3Token::Error(_)) => true,
                            (Aljam3Token::DefPermission, Aljam3Token::Data(_)) => true,
                            (Aljam3Token::DefCollector, Aljam3Token::Collector(_)) => true,
                            (Aljam3Token::DefConstructor, Aljam3Token::Constructor(_)) => true,
                            (Aljam3Token::DefComment, Aljam3Token::CommentText(_)) => true,
                            _ => false,
                        };

                        if !is_valid_target {
                            report.add_error(ValidationError {
                                context_snippets: vec![],
                                code: "PGE01049".to_string(),
                                name: "Invalid Definition Target".to_string(),
                                message: format!("The definition marker is followed by an invalid or unexpected token: {:?}", target_token),
                                line: ctx.tokens[i+1].line,
                                col: ctx.tokens[i+1].col,
                                snippet: get_snippet(ctx.tokens[i+1].line, ctx.lines),
                                help: Some(get_def_target_help(token_val, target_token)),
                            });
                        }
                    }
                }
            }
        }
    }
}
