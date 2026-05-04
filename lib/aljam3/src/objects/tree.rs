use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use regex::Regex;
use super::schema::Aljam3Schema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aljam3DataState {
    Declared,
    Default,
    Final,
    Failed,
    Released,
}

/// The actual underlying data payload
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aljam3LeafValue {
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
pub enum Aljam3DataLeaf {
    /// The leaf path exists but has no data. The Schema Datatype MUST be known.
    Declared { datatype: String }, 
    
    /// The leaf holds a default value.
    Default(Aljam3LeafValue),
    
    /// The leaf holds a successfully validated, final value.
    Final(Aljam3LeafValue),
    
    /// The leaf encountered an error.
    Failed { error: String },
    
    /// The leaf's memory has been explicitly freed/dropped.
    Released,
}

impl Aljam3LeafValue {
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
pub struct Aljam3DataTree {
    pub schema: Arc<Aljam3Schema>,
    /// A flat map representing the tree, where keys are paths (e.g., `#Person.Address:Line1`).
    /// Branches exist implicitly via the keys. Leaves hold the actual values.
    pub nodes: HashMap<String, Aljam3DataLeaf>,
}

impl Aljam3DataTree {
    pub fn new(_state: Aljam3DataState, schema: Arc<Aljam3Schema>) -> Self {
        Self {
            schema,
            nodes: HashMap::new(),
        }
    }

    /// Dynamically aggregates the state of all leaves to determine the master tree state.
    pub fn state(&self) -> Aljam3DataState {
        let mut has_declared = false;
        let mut has_failed = false;
        let mut has_default = false;
        let mut has_final = false;

        if self.nodes.is_empty() {
            return Aljam3DataState::Declared;
        }

        for leaf in self.nodes.values() {
            match leaf {
                Aljam3DataLeaf::Declared { .. } => has_declared = true,
                Aljam3DataLeaf::Failed { .. } => has_failed = true,
                Aljam3DataLeaf::Default(_) => has_default = true,
                Aljam3DataLeaf::Final(_) => has_final = true,
                Aljam3DataLeaf::Released => {}
            }
        }

        // Priority resolution
        if has_declared {
            Aljam3DataState::Declared
        } else if has_failed {
            Aljam3DataState::Failed
        } else if has_default {
            Aljam3DataState::Default
        } else if has_final {
            Aljam3DataState::Final
        } else {
            Aljam3DataState::Released
        }
    }

    pub fn insert_string(&mut self, path: impl Into<String>, value: String, regex: String) -> Result<(), String> {
        match Aljam3LeafValue::new_string(value, regex) {
            Ok(v) => {
                self.nodes.insert(path.into(), Aljam3DataLeaf::Final(v));
                Ok(())
            },
            Err(e) => {
                self.nodes.insert(path.into(), Aljam3DataLeaf::Failed { error: e.clone() });
                Err(e)
            }
        }
    }

    pub fn insert_enum(&mut self, path: impl Into<String>) {
        self.nodes.insert(path.into(), Aljam3DataLeaf::Final(Aljam3LeafValue::Enum));
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
            self.nodes.insert(path, Aljam3DataLeaf::Declared { datatype: datatype.to_string() });
        }
    }

    /// Implements %##Fields << #SomeEnum behavior for records.
    pub fn generate_record(&mut self, base_path: &str, field_names: &[String], datatype: &str) {
        for field in field_names {
            let path = format!("{}:{}", base_path, field);
            self.nodes.insert(path, Aljam3DataLeaf::Declared { datatype: datatype.to_string() });
        }
    }

    /// Implements ##Dataframe behavior (2-level tree: L1 rows by range, L2 columns by enum).
    pub fn generate_dataframe(&mut self, base_path: &str, rows: usize, columns: &[String], datatype: &str) {
        for row in 0..rows {
            for col in columns {
                let path = format!("{}:{}:{}", base_path, row, col);
                self.nodes.insert(path, Aljam3DataLeaf::Declared { datatype: datatype.to_string() });
            }
        }
    }

    /// Implements ##Map behavior (and ##TimeSeries).
    pub fn generate_map(&mut self, _base_path: &str) {
        // No pre-generation required for dynamic keys.
    }

    /// Extracts a subtree by keeping only nodes that start with the given branch path,
    /// and stripping the prefix so they are relative to the subtree root.
    pub fn extract_subtree(&self, branch_path: &str) -> HashMap<String, Aljam3DataLeaf> {
        let mut subtree = HashMap::new();
        for (key, leaf) in &self.nodes {
            if key == branch_path {
                subtree.insert("".to_string(), leaf.clone());
            } else if key.starts_with(branch_path) {
                let remainder = &key[branch_path.len()..];
                if remainder.starts_with('.') || remainder.starts_with(':') || remainder.starts_with('<') {
                    subtree.insert(remainder[1..].to_string(), leaf.clone());
                }
            }
        }
        subtree
    }

    /// Gets all immediate children (next segment of the path) of a given branch path.
    pub fn get_first_level_branches(&self, branch_path: &str) -> HashSet<String> {
        let mut branches = HashSet::new();
        for key in self.nodes.keys() {
            if key.starts_with(branch_path) && key.len() > branch_path.len() {
                let remainder = &key[branch_path.len()..];
                if remainder.starts_with('.') || remainder.starts_with(':') || remainder.starts_with('<') {
                    let sub_remainder = &remainder[1..];
                    let next_delim = sub_remainder.find(|c| c == '.' || c == ':' || c == '<').unwrap_or(sub_remainder.len());
                    branches.insert(sub_remainder[..next_delim].to_string());
                }
            }
        }
        branches
    }
    
    /// Calculates depth based on ., :, and < delimiters
    pub fn calculate_depth(path: &str) -> usize {
        path.chars().filter(|c| *c == '.' || *c == ':' || *c == '<').count()
    }
}
