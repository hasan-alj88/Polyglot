use crate::lexer::token::PolyglotToken;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub struct PatternRule {
    pub label: &'static str,
    pub regex: &'static Regex,
    pub extractor: fn(&Captures, Option<&PolyglotToken>) -> Vec<PolyglotToken>,
}

lazy_static! {
    static ref RE_TYPED_VAR: Regex = Regex::new(r"^\$(?P<var>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z][a-zA-Z0-9]*)*)#(?P<type>[a-zA-Z][a-zA-Z0-9]*(?:\.[a-zA-Z][a-zA-Z0-9]*)*)").unwrap();
    static ref RE_STANDALONE_VAR: Regex = Regex::new(r"^\$(?P<var>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z][a-zA-Z0-9]*)*)").unwrap();

    // String literals with potential variable substitutions
    static ref RE_STRING: Regex = Regex::new(r#"^"(?P<inner>.*?)""#).unwrap();

    // Registry and Package definitions with optional version
    static ref RE_REGISTRY_PKG: Regex = Regex::new(r"^@(?P<reg>[a-zA-Z0-9]+:[a-zA-Z0-9]+)<(?P<pkg>[a-zA-Z0-9.]+)(?:@(?P<ver>[a-zA-Z0-9.]+))?").unwrap();

    // Generic packages
    static ref RE_PACKAGE: Regex = Regex::new(r"^@(?P<pkg>[a-zA-Z0-9.]+)").unwrap();

    // Environment
    static ref RE_ENVIRONMENT: Regex = Regex::new(r"^;(?P<env>[a-zA-Z0-9.]+)").unwrap();

    // Actions & Identifiers: pipelines, triggers, queues, wrappers
    static ref RE_ACTION_CALL: Regex = Regex::new(r#"^-(?P<ident>[a-zA-Z0-9.]+)(?P<has_string>"(?P<str>.*?)")?"#).unwrap();

    // Constructors
    // Also matched for standalone variables if they don't have `#type` or `""` attached
    static ref RE_CONSTRUCTOR: Regex = Regex::new(r#"^\$(?P<ident>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z][a-zA-Z0-9]*)*)(?P<has_string>"(?P<str>.*?)")?"#).unwrap();

    // IO Parameters
    static ref RE_INPUT_PARAM: Regex = Regex::new(r"^<(?P<param>[a-zA-Z0-9.]+)").unwrap();
    static ref RE_OUTPUT_PARAM: Regex = Regex::new(r"^>(?P<param>[a-zA-Z0-9.]+)").unwrap();

    // Operators
    static ref RE_PULL: Regex = Regex::new(r"^<<").unwrap();
    static ref RE_PUSH: Regex = Regex::new(r"^>>").unwrap();
    static ref RE_DEFAULT_PULL: Regex = Regex::new(r"^<~").unwrap();
    static ref RE_DEFAULT_PUSH: Regex = Regex::new(r"^~>").unwrap();
    static ref RE_FALLBACK_PULL_LEFT: Regex = Regex::new(r"^!<").unwrap();
    static ref RE_FALLBACK_PUSH_RIGHT: Regex = Regex::new(r"^!>").unwrap();

    // Compression Operators
    static ref RE_COMPRESSION: Regex = Regex::new(r"^(?P<op>=\?|=!\?|>\?|>\!\?|<\?|<\!\?|\*\?)").unwrap();

    // Range Operations
    static ref RE_RANGE: Regex = Regex::new(r#"^\?(?P<open>\[|\()(?P<from>[^,]+),(?P<to>[^\]\)]+)(?P<close>\]|\))"#).unwrap();

    // Collectors
    static ref RE_COLLECTOR: Regex = Regex::new(r"^\*(?P<coll>[a-zA-Z0-9.]+)").unwrap();

    // Data and Metadata
    static ref RE_ISOLATED_DATA: Regex = Regex::new(r"^#(?P<data>[a-zA-Z0-9.]+)").unwrap();
    static ref RE_METADATA: Regex = Regex::new(r"^%(?P<meta>[a-zA-Z0-9.]+)").unwrap();

    // Comments
    static ref RE_COMMENT_ACTION: Regex = Regex::new(r"^\[ \] *(?P<text>.*)").unwrap();
    static ref RE_COMMENT_DEF: Regex = Regex::new(r"^\{ \} *(?P<text>.*)").unwrap();
    static ref RE_COMMENT_IO: Regex = Regex::new(r"^\( \) *(?P<text>.*)").unwrap();

    // Invalid constructs
    static ref RE_INVALID_FIELD: Regex = Regex::new(r"^[.:]_[a-zA-Z0-9_]*").unwrap();
}

pub fn extract_inline_string(inner: &str, is_inline_token: bool) -> Vec<PolyglotToken> {
    let mut tokens = Vec::new();
    let mut current = inner;
    while let Some(start) = current.find("{$") {
        if start > 0 {
            if is_inline_token {
                tokens.push(PolyglotToken::InlineString(current[..start].to_string()));
            } else {
                tokens.push(PolyglotToken::StringLiteral(current[..start].to_string()));
            }
        }
        let after_start = &current[start + 2..];
        if let Some(end) = after_start.find("}") {
            let var_name = &after_start[..end];
            tokens.push(PolyglotToken::SubstituteVariable(var_name.to_string()));
            current = &after_start[end + 1..];
        } else {
            if is_inline_token {
                tokens.push(PolyglotToken::InlineString(current[start..].to_string()));
            } else {
                tokens.push(PolyglotToken::StringLiteral(current[start..].to_string()));
            }
            current = "";
            break;
        }
    }
    if !current.is_empty() {
        if is_inline_token {
            tokens.push(PolyglotToken::InlineString(current.to_string()));
        } else {
            tokens.push(PolyglotToken::StringLiteral(current.to_string()));
        }
    }
    tokens
}

pub fn get_patterns() -> Vec<PatternRule> {
    vec![
        PatternRule {
            label: "String_Literal",
            regex: &RE_STRING,
            extractor: |caps, _| extract_inline_string(caps.name("inner").unwrap().as_str(), false),
        },
        PatternRule {
            label: "Typed_Variable",
            regex: &RE_TYPED_VAR,
            extractor: |caps, _| {
                vec![
                    PolyglotToken::Variable(caps["var"].to_string()),
                    PolyglotToken::DataType(caps["type"].to_string()),
                ]
            },
        },
        PatternRule {
            label: "Constructor",
            regex: &RE_CONSTRUCTOR,
            extractor: |caps, _| {
                let mut tokens = Vec::new();
                let ident = caps.name("ident").unwrap().as_str().to_string();

                if let Some(_str_match) = caps.name("has_string") {
                    tokens.push(PolyglotToken::Constructor(ident));
                    let inner = caps.name("str").unwrap().as_str();
                    tokens.extend(extract_inline_string(inner, true));
                } else {
                    // Fall back to just Variable if it's standalone and matched here
                    tokens.push(PolyglotToken::Variable(ident));
                }
                tokens
            },
        },
        PatternRule {
            label: "Registry_And_Package",
            regex: &RE_REGISTRY_PKG,
            extractor: |caps, _| {
                let mut tokens = vec![
                    PolyglotToken::Registry(caps["reg"].to_string()),
                    PolyglotToken::PackageName(caps["pkg"].to_string()),
                ];
                if caps.name("ver").is_none() {
                    tokens.push(PolyglotToken::NoVersion);
                }
                tokens
            },
        },
        PatternRule {
            label: "Environment",
            regex: &RE_ENVIRONMENT,
            extractor: |caps, _| vec![PolyglotToken::Environment(caps["env"].to_string())],
        },
        PatternRule {
            label: "Package",
            regex: &RE_PACKAGE,
            extractor: |caps, _| vec![PolyglotToken::Package(caps["pkg"].to_string())],
        },
        PatternRule {
            label: "Pull_From",
            regex: &RE_PULL,
            extractor: |_, _| vec![PolyglotToken::PullFrom],
        },
        PatternRule {
            label: "Push_Into",
            regex: &RE_PUSH,
            extractor: |_, _| vec![PolyglotToken::PushInto],
        },
        PatternRule {
            label: "Input_Parameter",
            regex: &RE_INPUT_PARAM,
            extractor: |caps, _| vec![PolyglotToken::InputParameter(caps["param"].to_string())],
        },
        PatternRule {
            label: "Output_Parameter",
            regex: &RE_OUTPUT_PARAM,
            extractor: |caps, _| vec![PolyglotToken::OutputParameter(caps["param"].to_string())],
        },
        PatternRule {
            label: "Action_Call",
            regex: &RE_ACTION_CALL,
            extractor: |caps, ctx| {
                let mut tokens = Vec::new();
                let ident = caps.name("ident").unwrap().as_str().to_string();

                if let Some(_str_match) = caps.name("has_string") {
                    // Inline instruction directly attached to string
                    tokens.push(PolyglotToken::InlineInstruction(ident));
                    let inner = caps.name("str").unwrap().as_str();
                    tokens.extend(extract_inline_string(inner, true));
                } else {
                    let mut is_trigger = false;
                    let mut is_queue = false;
                    let mut is_wrapper = false;

                    if let Some(token) = ctx {
                        match token {
                            PolyglotToken::ActionTrigger => is_trigger = true,
                            PolyglotToken::ActionQueue => is_queue = true,
                            PolyglotToken::ActionWrapper => is_wrapper = true,
                            _ => {}
                        }
                    }

                    if is_trigger {
                        tokens.push(PolyglotToken::Trigger(ident));
                    } else if is_queue {
                        tokens.push(PolyglotToken::QueueConfig(ident));
                    } else if is_wrapper {
                        tokens.push(PolyglotToken::Wrapper(ident));
                    } else {
                        tokens.push(PolyglotToken::Pipeline(ident));
                    }
                }

                tokens
            },
        },
        PatternRule {
            label: "Collector",
            regex: &RE_COLLECTOR,
            extractor: |caps, _| vec![PolyglotToken::Collector(caps["coll"].to_string())],
        },
        PatternRule {
            label: "Isolated_Data",
            regex: &RE_ISOLATED_DATA,
            extractor: |caps, _| vec![PolyglotToken::Data(caps["data"].to_string())],
        },
        PatternRule {
            label: "MetaData",
            regex: &RE_METADATA,
            extractor: |caps, _| vec![PolyglotToken::MetaData(caps["meta"].to_string())],
        },
        PatternRule {
            label: "Invalid_Identifier",
            regex: &RE_INVALID_FIELD,
            extractor: |caps, _| vec![PolyglotToken::InvalidIdentifier(caps[0].to_string())],
        },
        PatternRule {
            label: "Action_Comment",
            regex: &RE_COMMENT_ACTION,
            extractor: |caps, _| {
                vec![
                    PolyglotToken::ActionComment,
                    PolyglotToken::TokSpace,
                    PolyglotToken::CommentText(caps["text"].to_string()),
                ]
            },
        },
        PatternRule {
            label: "Definition_Comment",
            regex: &RE_COMMENT_DEF,
            extractor: |caps, _| {
                vec![
                    PolyglotToken::DefComment,
                    PolyglotToken::TokSpace,
                    PolyglotToken::CommentText(caps["text"].to_string()),
                ]
            },
        },
        PatternRule {
            label: "IO_Comment",
            regex: &RE_COMMENT_IO,
            extractor: |caps, _| {
                vec![
                    PolyglotToken::IoComment,
                    PolyglotToken::TokSpace,
                    PolyglotToken::CommentText(caps["text"].to_string()),
                ]
            },
        },
        PatternRule {
            label: "Default_Pull",
            regex: &RE_DEFAULT_PULL,
            extractor: |_, _| vec![PolyglotToken::DefaultPullFrom],
        },
        PatternRule {
            label: "Default_Push",
            regex: &RE_DEFAULT_PUSH,
            extractor: |_, _| vec![PolyglotToken::DefaultPushInto],
        },
        PatternRule {
            label: "Fallback_Pull_Left",
            regex: &RE_FALLBACK_PULL_LEFT,
            extractor: |_, _| vec![PolyglotToken::FallBackPullFrom],
        },
        PatternRule {
            label: "Fallback_Push_Right",
            regex: &RE_FALLBACK_PUSH_RIGHT,
            extractor: |_, _| vec![PolyglotToken::FallBackPushInto],
        },
        PatternRule {
            label: "Compression",
            regex: &RE_COMPRESSION,
            extractor: |caps, _| {
                let op = caps.name("op").unwrap().as_str();
                let token = match op {
                    "=?" => PolyglotToken::IsItEqual,
                    "=!?" => PolyglotToken::IsItNotEqual,
                    ">?" => PolyglotToken::IsItGreaterThan,
                    ">!?" => PolyglotToken::IsItNotGreaterThan,
                    "<?" => PolyglotToken::IsItLessThan,
                    "<!?" => PolyglotToken::IsItNotLessThan,
                    "*?" => PolyglotToken::IsItOtherwise,
                    _ => unreachable!(),
                };
                vec![token]
            },
        },
        PatternRule {
            label: "Range",
            regex: &RE_RANGE,
            extractor: |caps, _| {
                let open = caps.name("open").unwrap().as_str();
                let open_token = match open {
                    "[" => PolyglotToken::IsItInRangeInclusiveFrom,
                    "(" => PolyglotToken::IsItInRangeExclusiveFrom,
                    _ => unreachable!(),
                };

                let close = caps.name("close").unwrap().as_str();
                let close_token = match close {
                    "]" => PolyglotToken::IsItInRangeInclusiveTo,
                    ")" => PolyglotToken::IsItInRangeExclusiveTo,
                    _ => unreachable!(),
                };

                vec![
                    open_token,
                    PolyglotToken::RangeFrom(caps["from"].to_string()),
                    PolyglotToken::RangeSeparator,
                    PolyglotToken::RangeTo(caps["to"].to_string()),
                    close_token,
                ]
            },
        },
    ]
}
