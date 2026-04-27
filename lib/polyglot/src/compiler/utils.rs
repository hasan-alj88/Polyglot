use crate::lexer::token::{PolyglotToken, Spanned};

pub fn get_snippet(line: usize, lines: &[&str]) -> Option<String> {
    if line > 0 && line <= lines.len() {
        Some(lines[line - 1].to_string())
    } else {
        None
    }
}

pub fn get_def_target_help(def_marker: &PolyglotToken, target_token: &PolyglotToken) -> String {
    let def_str = match def_marker {
        PolyglotToken::DefPackage => "{@}",
        PolyglotToken::DefData => "{#}",
        PolyglotToken::DefPipeline => "{-}",
        PolyglotToken::DefTrigger => "{T}",
        PolyglotToken::DefWrapper => "{W}",
        PolyglotToken::DefNative => "{N}",
        PolyglotToken::DefQueue => "{Q}",
        PolyglotToken::DefError => "{!}",
        PolyglotToken::DefPermission => "{_}",
        PolyglotToken::DefCollector => "{*}",
        PolyglotToken::DefConstructor => "{$}",
        _ => "{?}",
    };

    let (expected_prefix, expected_type) = match def_marker {
        PolyglotToken::DefPackage => ("`@`", "Package or Registry"),
        PolyglotToken::DefData => ("`#`, `##`, or `###`", "Data, Array, or Table"),
        PolyglotToken::DefPipeline => ("`-`", "pipeline"),
        PolyglotToken::DefTrigger => ("`-`", "trigger"),
        PolyglotToken::DefWrapper => ("`-`", "wrapper"),
        PolyglotToken::DefNative => ("`N~`", "native function"),
        PolyglotToken::DefQueue => ("`#`", "queue configuration"),
        PolyglotToken::DefError => ("`!`", "error definition"),
        PolyglotToken::DefPermission => ("`_` or `__`", "permission"),
        PolyglotToken::DefCollector => ("`*`", "collector"),
        PolyglotToken::DefConstructor => ("`$`", "constructor or variable"),
        _ => ("the correct", "object"),
    };

    let actual_info = match target_token {
        PolyglotToken::Variable(_) => "`$` for variables (or constructors)",
        PolyglotToken::Constructor(_) => "`$` for constructors (or variables)",
        PolyglotToken::Data(_) => "`#` for data",
        PolyglotToken::Pipeline(_) => "`-` for pipelines",
        PolyglotToken::Registry(_) => "`@` for registries",
        PolyglotToken::Package(_) | PolyglotToken::PackageName(_) => "`@` for packages",
        PolyglotToken::Trigger(_) => "`-` for triggers",
        PolyglotToken::Wrapper(_) => "`-` for wrappers",
        PolyglotToken::QueueConfig(_) => "`#` for queue configs",
        PolyglotToken::Error(_) => "`!` for errors",
        PolyglotToken::Collector(_) => "`*` for collectors",
        PolyglotToken::MisplacedMarker(_) => "a structural marker",
        PolyglotToken::UnknownPolyglotObject(s) => return format!("`{}` defines a {} with {} prefix, not an unknown object: `{}`", def_str, expected_type, expected_prefix, s),
        _ => "the provided token",
    };

    format!("`{}` defines a {} with {} prefix, not {}", def_str, expected_type, expected_prefix, actual_info)
}

pub struct AnalysisContext<'a> {
    pub tokens: &'a [Spanned<PolyglotToken>],
    pub lines: &'a [&'a str],
    pub token_contexts: Vec<Vec<(PolyglotToken, usize)>>,
}

impl<'a> AnalysisContext<'a> {
    pub fn new(tokens: &'a [Spanned<PolyglotToken>], lines: &'a [&'a str]) -> Self {
        let mut token_contexts = Vec::with_capacity(tokens.len());
        let mut current_scope = 0;
        let mut context_stack: Vec<(PolyglotToken, usize)> = Vec::new();

        for token in tokens {
            if let PolyglotToken::Scope(s) = token.value {
                current_scope = s;
            }

            let is_def_marker = match token.value {
                PolyglotToken::DefPackage | PolyglotToken::DefData | PolyglotToken::DefPipeline
                | PolyglotToken::DefTrigger | PolyglotToken::DefWrapper | PolyglotToken::DefNative
                | PolyglotToken::DefQueue | PolyglotToken::DefError | PolyglotToken::DefPermission
                | PolyglotToken::DefCollector | PolyglotToken::DefConstructor | PolyglotToken::DefComment => true,
                _ => false,
            };

            let is_action_marker = match token.value {
                PolyglotToken::ActionRegistry | PolyglotToken::ActionExecSeq | PolyglotToken::ActionExecPar 
                | PolyglotToken::ActionExecBg | PolyglotToken::ActionDataLoad | PolyglotToken::ActionTypeBind 
                | PolyglotToken::ActionCondSwitch | PolyglotToken::ActionError | PolyglotToken::ActionTrigger 
                | PolyglotToken::ActionQueue | PolyglotToken::ActionWrapper | PolyglotToken::ActionScopeIn 
                | PolyglotToken::ActionScopeOut | PolyglotToken::ActionDataAccessFixed 
                | PolyglotToken::ActionDataAccessFlex | PolyglotToken::ActionLogicalAnd | PolyglotToken::ActionLogicalOr 
                | PolyglotToken::ActionLogicalXor | PolyglotToken::ActionContinuation | PolyglotToken::ActionForeignCode 
                | PolyglotToken::ActionMetadata | PolyglotToken::ActionComment | PolyglotToken::ActionImport 
                | PolyglotToken::ActionCollector | PolyglotToken::ContinueActionLine => true,
                _ => false,
            };

            if is_def_marker || is_action_marker {
                if current_scope < context_stack.len() {
                    context_stack.truncate(current_scope);
                }
                while context_stack.len() < current_scope {
                    let pad = context_stack.last().cloned().unwrap_or((PolyglotToken::DefPipeline, 0));
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
