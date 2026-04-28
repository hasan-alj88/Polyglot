use super::tree::{PolyglotDataTree, PolyglotDataState, PolyglotDataLeaf, PolyglotLeafValue};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonResult {
    Match,
    Mismatch(String),
    SourceNotInstantiated,
}

pub struct DatatypeComparator;

impl DatatypeComparator {
    /// Compares two PolyglotDataTrees directionally.
    /// Evaluates if the `target` theoretical topology can match the `source` topology.
    pub fn is_compatible(target: &PolyglotDataTree, source: &PolyglotDataTree) -> ComparisonResult {
        // 1. Check if source is instantiated (Final or Default)
        let source_state = source.state();
        if source_state != PolyglotDataState::Final && source_state != PolyglotDataState::Default {
            return ComparisonResult::SourceNotInstantiated;
        }

        // 2. Extract keys to compare topologies
        let target_keys: HashSet<&String> = target.nodes.keys().collect();
        let source_keys: HashSet<&String> = source.nodes.keys().collect();

        // 3. Topology shape comparison
        // In this flattened structure, a topology match means they have the exact same path keys,
        // and the same enum variants defined (if applicable).
        if target_keys != source_keys {
            let mut missing_in_target: Vec<_> = source_keys.difference(&target_keys).collect();
            let mut missing_in_source: Vec<_> = target_keys.difference(&source_keys).collect();
            
            missing_in_target.sort();
            missing_in_source.sort();
            
            return ComparisonResult::Mismatch(format!(
                "Topology mismatch. Target is missing paths: {:?}. Source is missing paths: {:?}",
                missing_in_target, missing_in_source
            ));
        }

        // 4. Detailed Leaf Topology check (ensuring types of leaves match)
        for key in target_keys {
            let target_leaf = target.nodes.get(key).unwrap();
            let source_leaf = source.nodes.get(key).unwrap();
            
            let t_val = match target_leaf {
                PolyglotDataLeaf::Default(v) | PolyglotDataLeaf::Final(v) => Some(v),
                _ => None, // It's Declared, Failed, or Released
            };
            
            let s_val = match source_leaf {
                PolyglotDataLeaf::Default(v) | PolyglotDataLeaf::Final(v) => Some(v),
                _ => None,
            };

            // If target is Declared, it just structurally expects a value here.
            // If both have concrete values, verify they are structurally similar.
            if let (Some(t), Some(s)) = (t_val, s_val) {
                match (t, s) {
                    (PolyglotLeafValue::String { .. }, PolyglotLeafValue::String { .. }) => {},
                    (PolyglotLeafValue::Enum, PolyglotLeafValue::Enum) => {},
                    _ => {
                        return ComparisonResult::Mismatch(format!(
                            "Leaf type mismatch at path '{}'. Target and source have different leaf data structures.",
                            key
                        ));
                    }
                }
            }
        }

        ComparisonResult::Match
    }
}
