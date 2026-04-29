use crate::lexer::token::PolyglotToken;
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

            if let PolyglotToken::Scope(s) = token_val {
                current_scope = *s;
            }

            let is_def_marker = match token_val {
                PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefNative
                | PolyglotToken::DefQueue | PolyglotToken::DefError | PolyglotToken::DefPermission
                | PolyglotToken::DefCollector | PolyglotToken::DefConstructor | PolyglotToken::DefComment => true,
                _ => false,
            };

            if is_def_marker {
                // PGE01053: Definition Scope
                if current_scope != 0 && !matches!(token_val, PolyglotToken::DefComment) {
                    let marker_char = match token_val {
                        PolyglotToken::DefPackage => "@",
                        PolyglotToken::DefData => "#",
                        PolyglotToken::DefPipeline => "-",
                        PolyglotToken::DefTrigger => "T",
                        PolyglotToken::DefWrapper => "W",
                        PolyglotToken::DefNative => "N",
                        PolyglotToken::DefQueue => "Q",
                        PolyglotToken::DefError => "!",
                        PolyglotToken::DefPermission => "_",
                        PolyglotToken::DefCollector => "*",
                        PolyglotToken::DefConstructor => "$",
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
                    if let PolyglotToken::DefQueue = token_val {
                        if let PolyglotToken::QueueConfig(name) = target_token {
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
                    if !(matches!(token_val, PolyglotToken::DefComment) && matches!(target_token, PolyglotToken::TokSpace)) {
                        let is_valid_target = match (token_val, target_token) {
                            (PolyglotToken::DefPackage, PolyglotToken::Registry(_)) => true,
                            (PolyglotToken::DefPackage, PolyglotToken::PackageName(_)) => true,
                            (PolyglotToken::DefPackage, PolyglotToken::Package(_)) => true,
                            (PolyglotToken::DefData, PolyglotToken::Data(_)) => true,
                            (PolyglotToken::DefPipeline, PolyglotToken::Pipeline(_)) => true,
                            (PolyglotToken::DefTrigger, PolyglotToken::Trigger(_)) => true,
                            (PolyglotToken::DefTrigger, PolyglotToken::InlineInstruction(_)) => true,
                            (PolyglotToken::DefWrapper, PolyglotToken::Wrapper(_)) => true,
                            (PolyglotToken::DefQueue, PolyglotToken::QueueConfig(_)) => true,
                            (PolyglotToken::DefError, PolyglotToken::Error(_)) => true,
                            (PolyglotToken::DefPermission, PolyglotToken::Data(_)) => true,
                            (PolyglotToken::DefCollector, PolyglotToken::Collector(_)) => true,
                            (PolyglotToken::DefConstructor, PolyglotToken::Constructor(_)) => true,
                            (PolyglotToken::DefComment, PolyglotToken::CommentText(_)) => true,
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
