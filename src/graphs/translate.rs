use std::collections::{HashMap, HashSet};

pub fn graph_to_map(graph: &[(u32, &[u32])]) -> HashMap<u32, HashSet<u32>> {
    graph.iter()
         .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect()))
         .collect()
}

pub fn graph_to_map_ref<'a>(graph: &'a [(u32, &'a [u32])]) -> HashMap<&'a u32, HashSet<&'a u32>> {
    graph.iter().map(|(node, neighbors)| {
        let neighbor_set: HashSet<&'a u32> = neighbors.iter().map(|n| n).collect();
        (node, neighbor_set)
    }).collect()
}