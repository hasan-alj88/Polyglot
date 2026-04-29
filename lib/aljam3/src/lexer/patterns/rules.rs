use crate::lexer::token::Aljam3Token;
use regex::Captures;
use super::regexes::*;

pub struct PatternRule {
    pub label: &'static str,
    pub regex: &'static regex::Regex,
    pub extractor: fn(&Captures, Option<&Aljam3Token>) -> Vec<Aljam3Token>,
}

pub fn extract_inline_string<F>(inner: &str, token_constructor: F) -> Vec<Aljam3Token>
where
    F: Fn(String) -> Aljam3Token,
{
    let mut tokens = Vec::new();
    let mut current = inner;
    while let Some(start) = current.find("{$") {
        if start > 0 {
            tokens.push(token_constructor(current[..start].to_string()));
        }
        let after_start = &current[start + 2..];
        if let Some(end) = after_start.find("}") {
            let var_name = &after_start[..end];
            tokens.push(Aljam3Token::SubstituteVariable(var_name.to_string()));
            current = &after_start[end + 1..];
        } else {
            tokens.push(token_constructor(current[start..].to_string()));
            current = "";
            break;
        }
    }
    if !current.is_empty() {
        tokens.push(token_constructor(current.to_string()));
    }
    tokens
}

pub fn get_patterns() -> Vec<PatternRule> {
    vec![
        PatternRule {
            label: "String_Literal",
            regex: &RE_STRING,
            extractor: |caps, _| extract_inline_string(caps.name("inner").unwrap().as_str(), Aljam3Token::StringLiteral),
        },
        PatternRule {
            label: "Typed_Variable",
            regex: &RE_TYPED_VAR,
            extractor: |caps, _| {
                vec![
                    Aljam3Token::Variable(caps["var"].to_string()),
                    Aljam3Token::DataType(caps["type"].to_string()),
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
                    tokens.push(Aljam3Token::Constructor(ident));
                    let inner = caps.name("str").unwrap().as_str();
                    tokens.extend(extract_inline_string(inner, Aljam3Token::ConstructorInlineString));
                } else {
                    // Fall back to just Variable if it's standalone and matched here
                    tokens.push(Aljam3Token::Variable(ident));
                }
                tokens
            },
        },
        PatternRule {
            label: "Registry_And_Package",
            regex: &RE_REGISTRY_PKG,
            extractor: |caps, _| {
                let mut tokens = vec![
                    Aljam3Token::Registry(caps["reg"].to_string()),
                    Aljam3Token::PackageName(caps["pkg"].to_string()),
                ];
                if let Some(ver) = caps.name("ver") {
                    tokens.push(Aljam3Token::PackageVersion(format!(":{}", ver.as_str())));
                } else {
                    tokens.push(Aljam3Token::NoVersion);
                }
                tokens
            },
        },
        PatternRule {
            label: "Environment",
            regex: &RE_ENVIRONMENT,
            extractor: |caps, _| vec![Aljam3Token::Environment(caps["env"].to_string())],
        },
        PatternRule {
            label: "Package",
            regex: &RE_PACKAGE,
            extractor: |caps, _| vec![Aljam3Token::Package(caps["pkg"].to_string())],
        },
        PatternRule {
            label: "Pull_From",
            regex: &RE_PULL,
            extractor: |_, _| vec![Aljam3Token::PullFrom],
        },
        PatternRule {
            label: "Push_Into",
            regex: &RE_PUSH,
            extractor: |_, _| vec![Aljam3Token::PushInto],
        },
        PatternRule {
            label: "Input_Parameter",
            regex: &RE_INPUT_PARAM,
            extractor: |caps, _| vec![Aljam3Token::InputParameter(caps["param"].to_string())],
        },
        PatternRule {
            label: "Output_Parameter",
            regex: &RE_OUTPUT_PARAM,
            extractor: |caps, _| vec![Aljam3Token::OutputParameter(caps["param"].to_string())],
        },
        PatternRule {
            label: "Action_Call",
            regex: &RE_ACTION_CALL,
            extractor: |caps, ctx| {
                let mut tokens = Vec::new();
                let ident = caps.name("ident").unwrap().as_str().to_string();

                if let Some(_str_match) = caps.name("has_string") {
                    // Inline instruction directly attached to string
                    tokens.push(Aljam3Token::InlineInstruction(ident));
                    let inner = caps.name("str").unwrap().as_str();
                    tokens.extend(extract_inline_string(inner, Aljam3Token::InlineString));
                } else {
                    let mut is_trigger = false;
                    let mut is_queue = false;
                    let mut is_wrapper = false;

                    if let Some(token) = ctx {
                        match token {
                            Aljam3Token::ActionTrigger | Aljam3Token::DefTrigger => is_trigger = true,
                            Aljam3Token::ActionQueue | Aljam3Token::DefQueue => is_queue = true,
                            Aljam3Token::ActionWrapper | Aljam3Token::DefWrapper => is_wrapper = true,
                            _ => {}
                        }
                    }

                    if is_trigger {
                        tokens.push(Aljam3Token::Trigger(ident));
                    } else if is_queue {
                        tokens.push(Aljam3Token::QueueConfig(ident));
                    } else if is_wrapper {
                        tokens.push(Aljam3Token::Wrapper(ident));
                    } else {
                        tokens.push(Aljam3Token::Pipeline(ident));
                    }
                }

                tokens
            },
        },
        PatternRule {
            label: "Collector",
            regex: &RE_COLLECTOR,
            extractor: |caps, _| vec![Aljam3Token::Collector(caps["coll"].to_string())],
        },
        PatternRule {
            label: "Data_Type",
            regex: &RE_DATA_TYPE,
            extractor: |caps, _| vec![Aljam3Token::DataType(caps["type"].to_string())],
        },
        PatternRule {
            label: "Isolated_Data",
            regex: &RE_ISOLATED_DATA,
            extractor: |caps, _| vec![Aljam3Token::Data(caps["data"].to_string())],
        },
        PatternRule {
            label: "Data_Field",
            regex: &RE_DATA_FIELD,
            extractor: |caps, _| vec![Aljam3Token::DataField(caps["ident"].to_string())],
        },
        PatternRule {
            label: "Boolean_Predicate",
            regex: &RE_PREDICATE,
            extractor: |caps, _| {
                let mut tokens = Vec::new();
                let ident = caps.name("ident").unwrap().as_str().to_string();
                tokens.push(Aljam3Token::BooleanPredicate(ident));
                if let Some(_str_match) = caps.name("has_string") {
                    let inner = caps.name("str").unwrap().as_str();
                    tokens.extend(extract_inline_string(inner, Aljam3Token::InlineString));
                }
                tokens
            },
        },
        PatternRule {
            label: "MetaData",
            regex: &RE_METADATA,
            extractor: |caps, _| vec![Aljam3Token::MetaData(caps["meta"].to_string())],
        },
        PatternRule {
            label: "Invalid_Identifier",
            regex: &RE_INVALID_FIELD,
            extractor: |caps, _| vec![Aljam3Token::InvalidIdentifier(caps[0].to_string())],
        },
        PatternRule {
            label: "Action_Comment",
            regex: &RE_COMMENT_ACTION,
            extractor: |caps, _| {
                vec![
                    Aljam3Token::ActionComment,
                    Aljam3Token::TokSpace,
                    Aljam3Token::CommentText(caps["text"].to_string()),
                ]
            },
        },
        PatternRule {
            label: "Definition_Comment",
            regex: &RE_COMMENT_DEF,
            extractor: |caps, _| {
                vec![
                    Aljam3Token::DefComment,
                    Aljam3Token::TokSpace,
                    Aljam3Token::CommentText(caps["text"].to_string()),
                ]
            },
        },
        PatternRule {
            label: "IO_Comment",
            regex: &RE_COMMENT_IO,
            extractor: |caps, _| {
                vec![
                    Aljam3Token::IoComment,
                    Aljam3Token::TokSpace,
                    Aljam3Token::CommentText(caps["text"].to_string()),
                ]
            },
        },
        PatternRule {
            label: "Default_Pull",
            regex: &RE_DEFAULT_PULL,
            extractor: |_, _| vec![Aljam3Token::DefaultPullFrom],
        },
        PatternRule {
            label: "Default_Push",
            regex: &RE_DEFAULT_PUSH,
            extractor: |_, _| vec![Aljam3Token::DefaultPushInto],
        },
        PatternRule {
            label: "Compression",
            regex: &RE_COMPRESSION,
            extractor: |caps, _| {
                let op = caps.name("op").unwrap().as_str();
                let token = match op {
                    "=?" => Aljam3Token::IsItEqual,
                    "=!?" => Aljam3Token::IsItNotEqual,
                    ">?" => Aljam3Token::IsItGreaterThan,
                    ">!?" => Aljam3Token::IsItNotGreaterThan,
                    "<?" => Aljam3Token::IsItLessThan,
                    "<!?" => Aljam3Token::IsItNotLessThan,
                    "*?" => Aljam3Token::IsItOtherwise,
                    _ => unreachable!(),
                };
                vec![token]
            },
        },
        PatternRule {
            label: "Fallback_Pull",
            regex: &RE_FALLBACK_PULL,
            extractor: |_, _| vec![Aljam3Token::FallBackPullFrom],
        },
        PatternRule {
            label: "Fallback_Push",
            regex: &RE_FALLBACK_PUSH,
            extractor: |_, _| vec![Aljam3Token::FallBackPushInto],
        },
        PatternRule {
            label: "Range",
            regex: &RE_RANGE,
            extractor: |caps, _| {
                let open = caps.name("open").unwrap().as_str();
                let open_token = match open {
                    "[" => Aljam3Token::IsItInRangeInclusiveFrom,
                    "(" => Aljam3Token::IsItInRangeExclusiveFrom,
                    _ => unreachable!(),
                };

                let close = caps.name("close").unwrap().as_str();
                let close_token = match close {
                    "]" => Aljam3Token::IsItInRangeInclusiveTo,
                    ")" => Aljam3Token::IsItInRangeExclusiveTo,
                    _ => unreachable!(),
                };

                vec![
                    open_token,
                    Aljam3Token::RangeFrom(caps["from"].to_string()),
                    Aljam3Token::RangeSeparator,
                    Aljam3Token::RangeTo(caps["to"].to_string()),
                    close_token,
                ]
            },
        },
    ]
}
