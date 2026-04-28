use std::collections::HashMap;
use std::sync::Arc;
use super::tree::PolyglotLeafData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveKind { All, One, Partial }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeafKind { String, Enum }

#[derive(Debug, Clone)]
pub enum SchemaPropertyValue {
    TerminalType(Arc<PolyglotSchema>), // Ref to another schema
    ChildrenSchema(Vec<Arc<PolyglotSchema>>), // Validates children against at least one schema
    Fields(Vec<String>),               // Required, no default
    Active(ActiveKind),                // Default: All
    Gap(bool),                         // Default: false
    CountMax(usize),                   // Default: usize::MAX (Inf)
    CountMin(usize),                   // Default: 1
    DepthMax(usize),                   // Default: usize::MAX (Inf)
    DepthMin(usize),                   // Default: 0
    Kind(LeafKind),                    // Default: String
}

#[derive(Debug, Clone)]
pub struct PolyglotSchema {
    /// Keys are formatted as `{branch}##{property}` or `{branch}###{property}`
    pub properties: HashMap<String, SchemaPropertyValue>,
}

impl PolyglotSchema {
    /// Creates an unconstrained schema
    pub fn new() -> Self {
        Self { properties: HashMap::new() }
    }
    
    pub fn insert_property(&mut self, branch: &str, prop_type: &str, prop_name: &str, value: SchemaPropertyValue) {
        let key = format!("{}{}{}", branch, prop_type, prop_name);
        self.properties.insert(key, value);
    }

    /// Central validation logic against a data tree instance
    pub fn validate_tree(&self, _tree_nodes: &HashMap<String, PolyglotLeafData>) -> Result<(), String> {
        // Validation sweeps per property type based on the defined spec
        // TODO: Implement full validation logic
        Ok(())
    }
}

// Implement PartialEq manually because Arc<PolyglotSchema> creates self-referential potential,
// but for our purposes, we can just check if they are the exact same Arc or have identical properties.
impl PartialEq for PolyglotSchema {
    fn eq(&self, other: &Self) -> bool {
        // Simple comparison for now, assuming schemas with same properties are equal.
        // For deep recursive schemas, this might need a more robust ID-based comparison.
        self.properties.len() == other.properties.len() // Placeholder
    }
}
impl Eq for PolyglotSchema {}
