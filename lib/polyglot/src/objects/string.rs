use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolyglotString {
    pub string_value: String,
    pub regex: String,
}

impl PolyglotString {
    pub fn new(string_value: impl Into<String>, regex: impl Into<String>) -> Self {
        Self {
            string_value: string_value.into(),
            regex: regex.into(),
        }
    }

    /// Validates the string value against the regex.
    /// Returns true if the regex is empty (representing #RawString) or if the regex matches.
    pub fn validate(&self) -> bool {
        if self.regex.is_empty() {
            return true;
        }

        match Regex::new(&self.regex) {
            Ok(compiled_regex) => compiled_regex.is_match(&self.string_value),
            Err(_) => false, // Invalid regex syntax is considered a failed validation here
        }
    }
}
