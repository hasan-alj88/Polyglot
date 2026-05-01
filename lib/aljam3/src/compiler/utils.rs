use crate::lexer::token::{Aljam3Token, Spanned};

pub fn get_snippet(line: usize, lines: &[&str]) -> Option<String> {
    if line > 0 && line <= lines.len() {
        Some(lines[line - 1].to_string())
    } else {
        None
    }
}

pub fn get_def_target_help(def_marker: &Aljam3Token, target_token: &Aljam3Token) -> String {
    let def_str = match def_marker {
        Aljam3Token::DefPackage => "{@}",
        Aljam3Token::DefData => "{#}",
        Aljam3Token::DefPipeline => "{-}",
        Aljam3Token::DefTrigger => "{T}",
        Aljam3Token::DefWrapper => "{W}",
        Aljam3Token::DefQueue => "{Q}",
        Aljam3Token::DefError => "{!}",
        Aljam3Token::DefPermission => "{_}",
        Aljam3Token::DefCollector => "{*}",
        Aljam3Token::DefConstructor => "{$}",
        _ => "{?}",
    };

    let (expected_prefix, expected_type) = match def_marker {
        Aljam3Token::DefPackage => ("`@`", "Package or Registry"),
        Aljam3Token::DefData => ("`#`, `##`, or `###`", "Data, Array, or Table"),
        Aljam3Token::DefPipeline => ("`-`", "pipeline"),
        Aljam3Token::DefTrigger => ("`-`", "trigger"),
        Aljam3Token::DefWrapper => ("`-`", "wrapper"),
        Aljam3Token::DefQueue => ("`#`", "queue configuration"),
        Aljam3Token::DefError => ("`!`", "error definition"),
        Aljam3Token::DefPermission => ("`_` or `__`", "permission"),
        Aljam3Token::DefCollector => ("`*`", "collector"),
        Aljam3Token::DefConstructor => ("`$`", "constructor or variable"),
        _ => ("the correct", "object"),
    };

    let actual_info = match target_token {
        Aljam3Token::Variable(_) => "`$` for variables (or constructors)",
        Aljam3Token::Constructor(_) => "`$` for constructors (or variables)",
        Aljam3Token::Data(_) => "`#` for data",
        Aljam3Token::Pipeline(_) => "`-` for pipelines",
        Aljam3Token::Registry(_) => "`@` for registries",
        Aljam3Token::Package(_) | Aljam3Token::PackageName(_) => "`@` for packages",
        Aljam3Token::Trigger(_) => "`-` for triggers",
        Aljam3Token::Wrapper(_) => "`-` for wrappers",
        Aljam3Token::QueueConfig(_) => "`#` for queue configs",
        Aljam3Token::Error(_) => "`!` for errors",
        Aljam3Token::Collector(_) => "`*` for collectors",
        Aljam3Token::MisplacedMarker(_) => "a structural marker",
        Aljam3Token::UnknownAljam3Object(s) => return format!("`{}` defines a {} with {} prefix, not an unknown object: `{}`", def_str, expected_type, expected_prefix, s),
        _ => "the provided token",
    };

    format!("`{}` defines a {} with {} prefix, not {}", def_str, expected_type, expected_prefix, actual_info)
}

pub struct AnalysisContext<'a> {
    pub tokens: &'a [Spanned<Aljam3Token>],
    pub lines: &'a [&'a str],
    pub token_contexts: Vec<Vec<(Aljam3Token, usize)>>,
}

impl<'a> AnalysisContext<'a> {
    pub fn new(tokens: &'a [Spanned<Aljam3Token>], lines: &'a [&'a str]) -> Self {
        let mut token_contexts = Vec::with_capacity(tokens.len());
        let mut current_scope = 0;
        let mut context_stack: Vec<(Aljam3Token, usize)> = Vec::new();

        for token in tokens {
            if let Aljam3Token::Scope(s) = token.value {
                current_scope = s;
            }

            let is_def_marker = match token.value {
                Aljam3Token::DefPackage | Aljam3Token::DefData | Aljam3Token::DefPipeline
                | Aljam3Token::DefTrigger | Aljam3Token::DefWrapper
                | Aljam3Token::DefQueue | Aljam3Token::DefError | Aljam3Token::DefPermission
                | Aljam3Token::DefCollector | Aljam3Token::DefConstructor | Aljam3Token::DefComment => true,
                _ => false,
            };

            let is_action_marker = match token.value {
                Aljam3Token::ActionRegistry | Aljam3Token::ActionExecSeq | Aljam3Token::ActionExecPar 
                | Aljam3Token::ActionExecBg | Aljam3Token::ActionDataLoad | Aljam3Token::ActionTypeBind 
                | Aljam3Token::ActionCondSwitch | Aljam3Token::ActionError | Aljam3Token::ActionTrigger 
                | Aljam3Token::ActionQueue | Aljam3Token::ActionWrapper | Aljam3Token::ActionScopeIn 
                | Aljam3Token::ActionScopeOut | Aljam3Token::ActionDataAccessFixed 
                | Aljam3Token::ActionDataAccessFlex | Aljam3Token::ActionLogicalAnd | Aljam3Token::ActionLogicalOr 
                | Aljam3Token::ActionLogicalXor | Aljam3Token::ActionContinuation | Aljam3Token::ActionForeignCode 
                | Aljam3Token::ActionMetadata | Aljam3Token::ActionComment | Aljam3Token::ActionImport 
                | Aljam3Token::ActionCollector | Aljam3Token::ContinueActionLine => true,
                _ => false,
            };

            if is_def_marker || is_action_marker {
                if current_scope < context_stack.len() {
                    context_stack.truncate(current_scope);
                }
                while context_stack.len() < current_scope {
                    let pad = context_stack.last().cloned().unwrap_or((Aljam3Token::DefPipeline, 0));
                    context_stack.push(pad);
                }
                context_stack.push((token.value.clone(), token.line));
            }

            token_contexts.push(context_stack.clone());
        }

        AnalysisContext {
            tokens,
            lines,
            token_contexts,
        }
    }
}
