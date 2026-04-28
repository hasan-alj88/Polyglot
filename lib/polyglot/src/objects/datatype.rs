use super::tree::{PolyglotDataTree, PolyglotDataState, PolyglotLeafData};
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
        if source.state != PolyglotDataState::Final && source.state != PolyglotDataState::Default {
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

            match (target_leaf, source_leaf) {
                (PolyglotLeafData::StringData(_), PolyglotLeafData::StringData(_)) => {
                    // Topologies match (both are strings at this path)
                }
                (
                    PolyglotLeafData::EnumSelector { valid_variants: target_vars, .. },
                    PolyglotLeafData::EnumSelector { valid_variants: source_vars, .. }
                ) => {
                    // Enum topology: valid variants must match to be considered same topology
                    let mut t_vars = target_vars.clone();
                    let mut s_vars = source_vars.clone();
                    t_vars.sort();
                    s_vars.sort();

                    if t_vars != s_vars {
                        return ComparisonResult::Mismatch(format!(
                            "Enum topology mismatch at path '{}'. Target variants: {:?}, Source variants: {:?}",
                            key, t_vars, s_vars
                        ));
                    }
                }
                _ => {
                    return ComparisonResult::Mismatch(format!(
                        "Leaf type mismatch at path '{}'. Target and source have different leaf data structures.",
                        key
                    ));
                }
            }
        }

        ComparisonResult::Match
    }
}
