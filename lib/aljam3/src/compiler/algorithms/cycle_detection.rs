use crate::lexer::token::{Aljam3Token, Spanned};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

pub fn detect_cycles(tokens: &[Spanned<Aljam3Token>]) -> Vec<Vec<String>> {
    let mut nodes = HashSet::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    
    let mut current_pipeline: Option<String> = None;
    
    // Pass 1: Build graph
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i].value {
            Aljam3Token::DefPipeline => {
                if i + 1 < tokens.len() {
                    if let Aljam3Token::Pipeline(name) = &tokens[i+1].value {
                        current_pipeline = Some(name.clone());
                        nodes.insert(name.clone());
                        edges.entry(name.clone()).or_insert_with(Vec::new);
                    }
                }
            }
            Aljam3Token::ActionExecSeq | Aljam3Token::ActionExecPar | Aljam3Token::ActionExecBg => {
                if let Some(parent) = &current_pipeline {
                    if i + 1 < tokens.len() {
                        if let Aljam3Token::Pipeline(target) = &tokens[i+1].value {
                            // Exclude aj3lib standard pipelines
                            if !target.starts_with("T.") && !target.starts_with("Q.") && !target.starts_with("W.") {
                                edges.get_mut(parent).unwrap().push(target.clone());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    // Pass 2: DFS Cycle Detection
    let mut color: HashMap<String, Color> = nodes.iter().map(|n| (n.clone(), Color::White)).collect();
    let mut parent_map: HashMap<String, String> = HashMap::new();
    let mut cycles = Vec::new();
    
    for node in &nodes {
        if color[node] == Color::White {
            dfs(node, &nodes, &edges, &mut color, &mut parent_map, &mut cycles);
        }
    }
    
    cycles
}

fn dfs(
    node: &String,
    nodes: &HashSet<String>,
    edges: &HashMap<String, Vec<String>>,
    color: &mut HashMap<String, Color>,
    parent_map: &mut HashMap<String, String>,
    cycles: &mut Vec<Vec<String>>
) {
    color.insert(node.clone(), Color::Gray);
    
    if let Some(neighbors) = edges.get(node) {
        for neighbor in neighbors {
            // Only traverse intra-package nodes (if it's not in `nodes`, it's an external import or mocked)
            if !nodes.contains(neighbor) {
                continue;
            }
            
            let neighbor_color = color.get(neighbor).copied().unwrap_or(Color::White);
            
            if neighbor_color == Color::Gray {
                // Back edge found! Extract cycle path
                let mut path = vec![node.clone()];
                let mut current = node.clone();
                while current != *neighbor {
                    if let Some(p) = parent_map.get(&current) {
                        path.push(p.clone());
                        current = p.clone();
                    } else {
                        break;
                    }
                }
                path.reverse();
                path.push(neighbor.clone());
                cycles.push(path);
            } else if neighbor_color == Color::White {
                parent_map.insert(neighbor.clone(), node.clone());
                dfs(neighbor, nodes, edges, color, parent_map, cycles);
            }
        }
    }
    
    color.insert(node.clone(), Color::Black);
}
