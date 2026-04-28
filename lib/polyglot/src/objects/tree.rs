use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use super::string::PolyglotString;
use super::schema::PolyglotSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolyglotDataState {
    Declared,
    Default,
    Final,
    Failed,
    Released,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolyglotLeafData {
    StringData(PolyglotString),
    EnumSelector {
        valid_variants: Vec<String>,
        active_variant: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyglotDataTree {
    pub state: PolyglotDataState,
    pub schema: Arc<PolyglotSchema>,
    /// A flat map representing the tree, where keys are paths (e.g., `#Person.Address:Line1`).
    /// Branches exist implicitly via the keys. Leaves hold the actual values.
    pub nodes: HashMap<String, PolyglotLeafData>,
}

impl PolyglotDataTree {
    pub fn new(state: PolyglotDataState, schema: Arc<PolyglotSchema>) -> Self {
        Self {
            state,
            schema,
            nodes: HashMap::new(),
        }
    }

    pub fn insert_string(&mut self, path: impl Into<String>, value: PolyglotString) {
        self.nodes.insert(path.into(), PolyglotLeafData::StringData(value));
    }

    pub fn insert_enum(&mut self, path: impl Into<String>, valid_variants: Vec<String>, active_variant: Option<String>) {
        self.nodes.insert(path.into(), PolyglotLeafData::EnumSelector { valid_variants, active_variant });
    }

    // pub fn to_json(&self) -> Result<String, serde_json::Error> {
    //     serde_json::to_string_pretty(self)
    // }

    // pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
    //     serde_json::from_str(json)
    // }

    /// Implements %##Fields << #Range behavior for multi-dimensional numeric arrays.
    /// Calculates Cartesian product of ranges to stamp out multi-level keys.
    pub fn generate_array(&mut self, base_path: &str, dimensions: usize, range: usize) {
        if dimensions == 0 || range == 0 {
            return;
        }

        let mut current_paths = vec![base_path.to_string()];

        for dim in 0..dimensions {
            let mut next_paths = Vec::new();
            for path in &current_paths {
                for i in 0..range {
                    let sep = if dim == 0 { ":" } else { "<" };
                    next_paths.push(format!("{}{}{}", path, sep, i));
                }
            }
            current_paths = next_paths;
        }

        for path in current_paths {
            // Placeholder for an uninitialized leaf
            self.nodes.insert(path, PolyglotLeafData::StringData(PolyglotString::new("", "")));
        }
    }

    /// Implements %##Fields << #SomeEnum behavior for records.
    /// Stamps out keys using the provided enum variants where all fields are considered active.
    pub fn generate_record(&mut self, base_path: &str, field_names: &[String]) {
        for field in field_names {
            let path = format!("{}:{}", base_path, field);
            // Placeholder for an uninitialized leaf
            self.nodes.insert(path, PolyglotLeafData::StringData(PolyglotString::new("", "")));
        }
    }
}
