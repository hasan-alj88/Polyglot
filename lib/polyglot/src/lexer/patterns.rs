//! # Polyglot Pattern Registry
//! 
//! This module defines the decoupled EBNF macro extractors used by the Lexer's 4-Phase Algorithm.
//! 
//! ## How to Add / Remove / Modify Patterns
//! 
//! To support future syntax changes safely, patterns are isolated into this registry. The Lexer
//! sequentially tests each pattern top-to-bottom.
//! 
//! ### 1. Adding a Pattern
//! 1. **Add the Regex:** Declare a new `Lazy` regex in the `lazy_static!` block. 
//!    - Ensure it starts with `^` to match the beginning of the remaining string.
//!    - Use named capture groups (e.g. `(?P<group_name>...)`) to extract values.
//! 2. **Register the Rule:** Append a new `PatternRule` to the `vec!` in `get_patterns()`.
//!    - `label`: A clear human-readable name matching the EBNF logic (e.g. "Assign_Data").
//!    - `regex`: A reference to your new static regex.
//!    - `extractor`: A closure `|caps| vec![...]` that converts the captured strings into an array of `PolyglotToken`s.
//! 
//! ### 2. Modifying a Pattern
//! - If the grammatical structure changes, locate the `lazy_static!` definition and update the Regex string.
//! - If the token output changes, locate the specific `extractor` closure in `get_patterns()` and change the yielded array sequence.
//! 
//! ### 3. Removing a Pattern
//! - Simply delete the `PatternRule` from the `get_patterns()` array, and remove its static Regex definition.
//!
//! **Warning on Precedence:** The engine evaluates patterns top-to-bottom. Place more specific/greedy macro patterns (like `Assign_Data`) ABOVE generic catch-alls (like `Standalone_Variable`)!

use crate::lexer::token::PolyglotToken;
use regex::{Captures, Regex};
use lazy_static::lazy_static;

pub struct PatternRule {
    pub label: &'static str,
    pub regex: &'static Regex,
    pub extractor: fn(&Captures) -> Vec<PolyglotToken>,
}

lazy_static! {
    static ref RE_TYPED_VAR: Regex = Regex::new(r"^\$(?P<var>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z][a-zA-Z0-9]*)*)#(?P<type>[a-zA-Z][a-zA-Z0-9]*(?:\.[a-zA-Z][a-zA-Z0-9]*)*)").unwrap();
    static ref RE_ASSIGN_DATA: Regex = Regex::new(r"^\$(?P<var>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z][a-zA-Z0-9]*)*) +<< +#(?P<data>[a-zA-Z][a-zA-Z0-9]*(?:\.[a-zA-Z][a-zA-Z0-9]*)*)").unwrap();
    static ref RE_RAISE_ERROR: Regex = Regex::new(r"^>> +!(?P<err>[a-zA-Z][a-zA-Z0-9]*(?:\.[a-zA-Z][a-zA-Z0-9]*)*)").unwrap();
    static ref RE_ISOLATED_DATA: Regex = Regex::new(r"^#(?P<data>[a-zA-Z][a-zA-Z0-9]*(?:\.[a-zA-Z][a-zA-Z0-9]*)*)").unwrap();
    static ref RE_STANDALONE_VAR: Regex = Regex::new(r"^\$(?P<var>[a-zA-Z][a-zA-Z0-9]*(?:[.:][a-zA-Z][a-zA-Z0-9]*)*)").unwrap();
}

pub fn get_patterns() -> Vec<PatternRule> {
    vec![
        PatternRule {
            label: "Typed_Variable",
            regex: &RE_TYPED_VAR,
            extractor: |caps| vec![
                PolyglotToken::Variable(caps["var"].to_string()),
                PolyglotToken::DataType(caps["type"].to_string()),
            ],
        },
        PatternRule {
            label: "Assign_Data",
            regex: &RE_ASSIGN_DATA,
            extractor: |caps| vec![
                PolyglotToken::Variable(caps["var"].to_string()),
                PolyglotToken::TokSpace,
                PolyglotToken::OpPushLeft,
                PolyglotToken::TokSpace,
                PolyglotToken::Data(caps["data"].to_string()),
            ],
        },
        PatternRule {
            label: "Raise_Error_Macro",
            regex: &RE_RAISE_ERROR,
            extractor: |caps| vec![
                PolyglotToken::OpPushRight,
                PolyglotToken::TokSpace,
                PolyglotToken::Error(caps["err"].to_string()),
            ],
        },
        PatternRule {
            label: "Isolated_Data",
            regex: &RE_ISOLATED_DATA,
            extractor: |caps| vec![
                PolyglotToken::Data(caps["data"].to_string()),
            ],
        },
        PatternRule {
            label: "Standalone_Variable",
            regex: &RE_STANDALONE_VAR,
            extractor: |caps| vec![
                PolyglotToken::Variable(caps["var"].to_string()),
            ],
        },
    ]
}
