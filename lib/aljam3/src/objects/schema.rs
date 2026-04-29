use std::collections::HashMap;
use std::sync::Arc;
use super::tree::{Aljam3DataLeaf, Aljam3LeafValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveKind { All, One, Partial }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafKind { String, Enum }

#[derive(Debug, Clone)]
pub enum SchemaPropertyValue {
    TerminalType(Arc<Aljam3Schema>), // Ref to another schema
    ChildrenSchema(Vec<Arc<Aljam3Schema>>), // Validates children against at least one schema
    Fields(Vec<String>),               // Required, no default
    Active(ActiveKind),                // Default: All
    Gap(bool),                         // Default: false
    CountMax(usize),                   // Default: usize::MAX (Inf)
    CountMin(usize),                   // Default: 1
    DepthMax(usize),                   // Default: usize::MAX (Inf)
    DepthMin(usize),                   // Default: 0
    Kind(LeafKind),                    // Default: String
    Alias(Vec<HashMap<String, String>>), // Maps alias -> datatree keys. Default: Empty Vec.
    Unique(bool),                      // ALL leafs of this branch must be unique. Default: false.
}

#[derive(Debug, Clone)]
pub struct Aljam3Schema {
    /// Keys are formatted as `{branch}##{property}` or `{branch}###{property}`
    pub properties: HashMap<String, SchemaPropertyValue>,
}

impl Aljam3Schema {
    /// Creates an unconstrained schema
    pub fn new() -> Self {
        Self { properties: HashMap::new() }
    }
    
    pub fn insert_property(&mut self, branch: &str, prop_type: &str, prop_name: &str, value: SchemaPropertyValue) {
        let key = format!("{}{}{}", branch, prop_type, prop_name);
        self.properties.insert(key, value);
    }

    /// Resolves a potential alias back to its canonical branch key.
    pub fn resolve_alias(&self, potential_alias: &str) -> String {
        for prop in self.properties.values() {
            if let SchemaPropertyValue::Alias(alias_vec) = prop {
                for alias_map in alias_vec {
                    // Exact match
                    if let Some(canonical) = alias_map.get(potential_alias) {
                        return canonical.clone();
                    }
                    // Segment match (e.g., "#Bound.inf" -> "#Bound.Inf")
                    for (alias, canonical) in alias_map {
                        if potential_alias.ends_with(&format!(".{}", alias)) {
                            return potential_alias.replace(&format!(".{}", alias), &format!(".{}", canonical));
                        }
                        if potential_alias.ends_with(&format!(":{}", alias)) {
                            return potential_alias.replace(&format!(":{}", alias), &format!(":{}", canonical));
                        }
                    }
                }
            }
        }
        potential_alias.to_string()
    }

    /// Retrieves a property, automatically checking aliases if the direct key is not found.
    pub fn get_property(&self, branch: &str, prop_type: &str, prop_name: &str) -> Option<&SchemaPropertyValue> {
        let key = format!("{}{}{}", branch, prop_type, prop_name);
        
        // Fast path: direct lookup
        if let Some(val) = self.properties.get(&key) {
            return Some(val);
        }
        
        // Fallback: If key not in the hashmap check the aliases
        let resolved_branch = self.resolve_alias(branch);
        if resolved_branch != branch {
            let resolved_key = format!("{}{}{}", resolved_branch, prop_type, prop_name);
            return self.properties.get(&resolved_key);
        }
        
        None
    }

    /// Central validation logic against a data tree instance
    pub fn validate_tree(&self, tree_nodes: &HashMap<String, Aljam3DataLeaf>) -> Result<(), String> {
        // Validation sweeps per property type based on the defined spec
        
        for (path, leaf) in tree_nodes {
            let is_enum = match leaf {
                Aljam3DataLeaf::Default(Aljam3LeafValue::Enum) |
                Aljam3DataLeaf::Final(Aljam3LeafValue::Enum) => true,
                _ => false,
            };
            
            if is_enum {
                // 1. Resolve the runtime path (e.g., "%#:Person:0.Role.Admin") 
                //    back to its schema branch (e.g., "#Person.Role")
                // TODO: Implement actual runtime-to-schema path resolution
                let schema_branch = Self::resolve_to_schema_path_placeholder(path);
                
                // 2. Look up the %##Active property for this branch
                let active_prop = self.get_property(&schema_branch, "##", "Active");
                
                // 3. Enforce the invariant: Enums MUST belong to an ActiveKind::One branch
                match active_prop {
                    Some(SchemaPropertyValue::Active(ActiveKind::One)) => {
                        // Valid! This branch enforces exactly 1 active child.
                    },
                    Some(SchemaPropertyValue::Active(kind)) => {
                        // INVALID! The schema allows Active > 1 (All or Partial)
                        return Err(format!(
                            "Validation Error: Path '{}' is an Enum variant, but its parent branch allows ActiveKind::{:?}. Enums require ActiveKind::One.", 
                            path, kind
                        ));
                    },
                    None => {
                        // The spec says default %##Active is ActiveKind::All
                        // So if it's missing, it defaults to All, which is invalid for Enums.
                        return Err(format!(
                            "Validation Error: Path '{}' is an Enum variant, but its parent branch has no %##Active property (defaults to All). Enums require ActiveKind::One.", 
                            path
                        ));
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }

    /// Placeholder for converting a runtime path like "%#:Person:0.Role.Admin" to "#Person.Role"
    fn resolve_to_schema_path_placeholder(runtime_path: &str) -> String {
        // Just a dummy implementation for now to make it compile.
        // Needs real parsing logic to strip instances and terminal nodes.
        runtime_path.to_string()
    }
}

// Implement PartialEq manually because Arc<Aljam3Schema> creates self-referential potential,
// but for our purposes, we can just check if they are the exact same Arc or have identical properties.
impl PartialEq for Aljam3Schema {
    fn eq(&self, other: &Self) -> bool {
        // Simple comparison for now, assuming schemas with same properties are equal.
        // For deep recursive schemas, this might need a more robust ID-based comparison.
        self.properties.len() == other.properties.len() // Placeholder
    }
}
impl Eq for Aljam3Schema {}
