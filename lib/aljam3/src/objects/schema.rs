use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use super::tree::{Aljam3DataTree, Aljam3DataLeaf, Aljam3LeafValue};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveKind { All, One, Partial }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafKind { String, Enum }

#[derive(Debug, Clone)]
pub enum SchemaPropertyValue {
    TerminalSchema(Arc<Aljam3Schema>), // Ref to another schema
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
    pub fn validate_tree(&self, tree: &Aljam3DataTree) -> Result<(), String> {
        // Validation sweeps per property type based on the defined spec
        
        for (path, leaf) in &tree.nodes {
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

        // New property validations
        for (key, prop) in &self.properties {
            let delimiter = if key.contains("###") { "###" } else if key.contains("##") { "##" } else { "" };
            if delimiter.is_empty() { continue; }
            
            let parts: Vec<&str> = key.split(delimiter).collect();
            if parts.is_empty() { continue; }
            let branch_path = parts[0];

            match prop {
                SchemaPropertyValue::TerminalSchema(ts) => self.verify_terminal_schema(tree, branch_path, ts)?,
                SchemaPropertyValue::ChildrenSchema(cs) => self.verify_children_schema(tree, branch_path, cs)?,
                SchemaPropertyValue::Active(ak) => self.verify_active(tree, branch_path, ak)?,
                SchemaPropertyValue::Gap(allow) => self.verify_gap(tree, branch_path, *allow)?,
                SchemaPropertyValue::CountMax(max) => self.verify_count(tree, branch_path, 0, *max)?,
                SchemaPropertyValue::CountMin(min) => self.verify_count(tree, branch_path, *min, usize::MAX)?,
                SchemaPropertyValue::DepthMax(max) => self.verify_depth(tree, branch_path, 0, *max)?,
                SchemaPropertyValue::DepthMin(min) => self.verify_depth(tree, branch_path, *min, usize::MAX)?,
                SchemaPropertyValue::Kind(k) => self.verify_kind(tree, branch_path, k)?,
                SchemaPropertyValue::Unique(u) => self.verify_unique(tree, branch_path, *u)?,
                _ => {} // Fields, Alias etc are handled implicitly or not structural checks
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

    pub fn verify_terminal_schema(&self, tree: &Aljam3DataTree, branch_path: &str, terminal_schema: &Arc<Aljam3Schema>) -> Result<(), String> {
        let extracted_subtree = tree.extract_subtree(branch_path);
        let temp_tree = Aljam3DataTree {
            schema: terminal_schema.clone(),
            nodes: extracted_subtree,
        };
        terminal_schema.validate_tree(&temp_tree)
    }

    pub fn verify_children_schema(&self, tree: &Aljam3DataTree, branch_path: &str, children_schemas: &[Arc<Aljam3Schema>]) -> Result<(), String> {
        let children = tree.get_first_level_branches(branch_path);
        for child in children {
            let child_path = if branch_path.is_empty() { child.clone() } else { format!("{}.{}", branch_path, child) };
            let child_subtree = tree.extract_subtree(&child_path);
            
            let mut any_valid = false;
            for schema in children_schemas {
                let temp_tree = Aljam3DataTree {
                    schema: schema.clone(),
                    nodes: child_subtree.clone(),
                };
                if schema.validate_tree(&temp_tree).is_ok() {
                    any_valid = true;
                    break;
                }
            }
            if !any_valid {
                return Err(format!("Child {} does not conform to any ChildrenSchema", child_path));
            }
        }
        Ok(())
    }

    pub fn verify_active(&self, tree: &Aljam3DataTree, branch_path: &str, active_kind: &ActiveKind) -> Result<(), String> {
        let children_count = tree.get_first_level_branches(branch_path).len();
        
        match active_kind {
            ActiveKind::One => {
                if children_count != 1 {
                    return Err(format!("Branch {} requires exactly one active child, but has {}", branch_path, children_count));
                }
            }
            ActiveKind::Partial => {
                let fields = match self.get_property(branch_path, "##", "Fields") {
                    Some(SchemaPropertyValue::Fields(f)) => f.len(),
                    _ => usize::MAX, // Can't fully bound without Fields
                };
                if children_count < 1 || children_count > fields {
                    return Err(format!("Branch {} requires partial active children (1 to {}), but has {}", branch_path, fields, children_count));
                }
            }
            ActiveKind::All => {
                let fields = match self.get_property(branch_path, "##", "Fields") {
                    Some(SchemaPropertyValue::Fields(f)) => f.len(),
                    _ => return Ok(()), // If no Fields specified, we assume any number is 'all'
                };
                if children_count != fields {
                    return Err(format!("Branch {} requires all {} children to be active, but has {}", branch_path, fields, children_count));
                }
            }
        }
        Ok(())
    }

    pub fn verify_gap(&self, tree: &Aljam3DataTree, branch_path: &str, allow_gap: bool) -> Result<(), String> {
        if allow_gap { return Ok(()); }
        
        let children = tree.get_first_level_branches(branch_path);
        let mut numeric_indices = Vec::new();
        
        for child in children {
            if let Ok(idx) = child.parse::<usize>() {
                numeric_indices.push(idx);
            } else {
                // If branches are non-numeric, return Ok
                return Ok(());
            }
        }
        
        if numeric_indices.is_empty() { return Ok(()); }
        numeric_indices.sort_unstable();
        
        let mut expected = 0;
        for &idx in &numeric_indices {
            if idx != expected {
                return Err(format!("Gap detected at {} in branch {}", expected, branch_path));
            }
            expected += 1;
        }
        Ok(())
    }

    pub fn verify_count(&self, tree: &Aljam3DataTree, branch_path: &str, min: usize, max: usize) -> Result<(), String> {
        let children_count = tree.get_first_level_branches(branch_path).len();
        if children_count < min || children_count > max {
            return Err(format!("Branch {} child count {} is out of bounds [{}, {}]", branch_path, children_count, min, max));
        }
        Ok(())
    }

    pub fn verify_depth(&self, tree: &Aljam3DataTree, branch_path: &str, min: usize, max: usize) -> Result<(), String> {
        let subtree = tree.extract_subtree(branch_path);
        for path in subtree.keys() {
            let depth = Aljam3DataTree::calculate_depth(path);
            if depth < min || depth > max {
                return Err(format!("Path depth {} in branch {} is out of bounds [{}, {}]", depth, branch_path, min, max));
            }
        }
        Ok(())
    }

    pub fn verify_kind(&self, tree: &Aljam3DataTree, branch_path: &str, kind: &LeafKind) -> Result<(), String> {
        let subtree = tree.extract_subtree(branch_path);
        for leaf in subtree.values() {
            match kind {
                LeafKind::String => {
                    if let Aljam3DataLeaf::Default(Aljam3LeafValue::Enum) | Aljam3DataLeaf::Final(Aljam3LeafValue::Enum) = leaf {
                        return Err(format!("Expected String kind, found Enum at branch {}", branch_path));
                    }
                }
                LeafKind::Enum => {
                    if let Aljam3DataLeaf::Default(Aljam3LeafValue::String { .. }) | Aljam3DataLeaf::Final(Aljam3LeafValue::String { .. }) = leaf {
                        return Err(format!("Expected Enum kind, found String at branch {}", branch_path));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn verify_unique(&self, tree: &Aljam3DataTree, branch_path: &str, must_be_unique: bool) -> Result<(), String> {
        if !must_be_unique { return Ok(()); }
        
        let subtree = tree.extract_subtree(branch_path);
        let mut seen = HashSet::new();
        
        for leaf in subtree.values() {
            if let Aljam3DataLeaf::Final(Aljam3LeafValue::String { value, .. }) | Aljam3DataLeaf::Default(Aljam3LeafValue::String { value, .. }) = leaf {
                if !seen.insert(value) {
                    return Err(format!("Duplicate leaf value '{}' found in unique branch {}", value, branch_path));
                }
            }
        }
        Ok(())
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
