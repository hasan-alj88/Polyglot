use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use regex::Regex;
use super::schema::PolyglotSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolyglotDataState {
    Declared,
    Default,
    Final,
    Failed,
    Released,
}

/// The actual underlying data payload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolyglotLeafValue {
    // Represents a purely structural Enum variant (no underlying data)
    Enum,
    
    // Represents a Value variant that carries string and regex properties
    String {
        value: String,
        regex: String,
    },
}

/// The state-aware leaf in the data tree
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolyglotDataLeaf {
    /// The leaf path exists but has no data. The Schema Datatype MUST be known.
    Declared { datatype: String }, 
    
    /// The leaf holds a default value.
    Default(PolyglotLeafValue),
    
    /// The leaf holds a successfully validated, final value.
    Final(PolyglotLeafValue),
    
    /// The leaf encountered an error.
    Failed { error: String },
    
    /// The leaf's memory has been explicitly freed/dropped.
    Released,
}

impl PolyglotLeafValue {
    /// Creates a new String variant with regex validation.
    /// Returns an error if the value does not match the regex.
    pub fn new_string(value: impl Into<String>, regex: impl Into<String>) -> Result<Self, String> {
        let value = value.into();
        let regex = regex.into();
        
        let leaf = Self::String { value, regex };
        
        if leaf.validate() {
            Ok(leaf)
        } else {
            Err(format!("Value does not match the provided regex"))
        }
    }

    /// Creates a new String variant without validation (for internal/trusted use)
    pub fn new_string_unchecked(value: impl Into<String>, regex: impl Into<String>) -> Self {
        Self::String {
            value: value.into(),
            regex: regex.into(),
        }
    }

    /// Validates the string value against the regex.
    /// Returns true if it's an Enum, or if it's a String and the regex is empty/matches.
    pub fn validate(&self) -> bool {
        match self {
            Self::Enum => true,
            Self::String { value, regex } => {
                if regex.is_empty() {
                    return true;
                }
                match Regex::new(regex) {
                    Ok(compiled_regex) => compiled_regex.is_match(value),
                    Err(_) => false, // Invalid regex syntax is considered a failed validation
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolyglotDataTree {
    pub schema: Arc<PolyglotSchema>,
    /// A flat map representing the tree, where keys are paths (e.g., `#Person.Address:Line1`).
    /// Branches exist implicitly via the keys. Leaves hold the actual values.
    pub nodes: HashMap<String, PolyglotDataLeaf>,
}

impl PolyglotDataTree {
    pub fn new(_state: PolyglotDataState, schema: Arc<PolyglotSchema>) -> Self {
        Self {
            schema,
            nodes: HashMap::new(),
        }
    }

    /// Dynamically aggregates the state of all leaves to determine the master tree state.
    pub fn state(&self) -> PolyglotDataState {
        let mut has_declared = false;
        let mut has_failed = false;
        let mut has_default = false;
        let mut has_final = false;

        if self.nodes.is_empty() {
            return PolyglotDataState::Declared;
        }

        for leaf in self.nodes.values() {
            match leaf {
                PolyglotDataLeaf::Declared { .. } => has_declared = true,
                PolyglotDataLeaf::Failed { .. } => has_failed = true,
                PolyglotDataLeaf::Default(_) => has_default = true,
                PolyglotDataLeaf::Final(_) => has_final = true,
                PolyglotDataLeaf::Released => {}
            }
        }

        // Priority resolution
        if has_declared {
            PolyglotDataState::Declared
        } else if has_failed {
            PolyglotDataState::Failed
        } else if has_default {
            PolyglotDataState::Default
        } else if has_final {
            PolyglotDataState::Final
        } else {
            PolyglotDataState::Released
        }
    }

    pub fn insert_string(&mut self, path: impl Into<String>, value: String, regex: String) -> Result<(), String> {
        match PolyglotLeafValue::new_string(value, regex) {
            Ok(v) => {
                self.nodes.insert(path.into(), PolyglotDataLeaf::Final(v));
                Ok(())
            },
            Err(e) => {
                self.nodes.insert(path.into(), PolyglotDataLeaf::Failed { error: e.clone() });
                Err(e)
            }
        }
    }

    pub fn insert_enum(&mut self, path: impl Into<String>) {
        self.nodes.insert(path.into(), PolyglotDataLeaf::Final(PolyglotLeafValue::Enum));
    }

    /// Implements %##Fields << #Range behavior for multi-dimensional numeric arrays.
    pub fn generate_array(&mut self, base_path: &str, dimensions: usize, range: usize, datatype: &str) {
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
            self.nodes.insert(path, PolyglotDataLeaf::Declared { datatype: datatype.to_string() });
        }
    }

    /// Implements %##Fields << #SomeEnum behavior for records.
    pub fn generate_record(&mut self, base_path: &str, field_names: &[String], datatype: &str) {
        for field in field_names {
            let path = format!("{}:{}", base_path, field);
            self.nodes.insert(path, PolyglotDataLeaf::Declared { datatype: datatype.to_string() });
        }
    }

    /// Implements ##Dataframe behavior (2-level tree: L1 rows by range, L2 columns by enum).
    pub fn generate_dataframe(&mut self, base_path: &str, rows: usize, columns: &[String], datatype: &str) {
        for row in 0..rows {
            for col in columns {
                let path = format!("{}:{}:{}", base_path, row, col);
                self.nodes.insert(path, PolyglotDataLeaf::Declared { datatype: datatype.to_string() });
            }
        }
    }

    /// Implements ##Map behavior (and ##TimeSeries).
    pub fn generate_map(&mut self, _base_path: &str) {
        // No pre-generation required for dynamic keys.
    }
}
