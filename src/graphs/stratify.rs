use std::collections::{HashMap, HashSet};

use crate::structs::vector3d::Vector3D;

type AdjDict = HashMap<u32, HashSet<u32>>;
type Verts = Vec<Vector3D>;

fn stratified_nodes(vects3d: &Verts) -> HashMap<i32, HashSet<u32>> {
    vects3d.iter()
        .map(|v| v.z)
        .filter(|&z| z < 0i32)
        .collect::<HashSet<i32>>()
        .into_iter()
        .map(|z| {
            let nodes = vects3d.iter().enumerate()
                .filter(|&(_, v)| v.z as i32 == z)
                .map(|(i, _)| i as u32)
                .collect::<HashSet<u32>>();
            (z, nodes)
        })
        .collect()
}

fn filter_graph(adj: &HashMap<u32, HashSet<u32>>, nodes: &HashSet<u32>) -> HashMap<u32, HashSet<u32>> {
    let filtered: HashMap<u32, HashSet<u32>> = adj.iter()
        .filter(|(k, _)| nodes.contains(k))
        .map(|(k, v)| (*k, v.intersection(nodes).copied().collect()))
        .collect();
    filtered
}

fn get_zlevel_length(stratified: &HashMap<i32, HashSet<u32>>) -> HashMap<i32, usize> {
    stratified.iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .collect()
}

pub fn shrink_adjacency(vects3d: &Verts, adj: &AdjDict) -> (AdjDict, HashMap<i32, usize>) {
    let stratified = stratified_nodes(vects3d);
    let nodes: HashSet<u32> = stratified[&(-1 as i32)].clone();
    let z_adj: HashMap<u32, HashSet<u32>> = filter_graph(&adj, &nodes);
    let z_length = get_zlevel_length(&stratified);
    (z_adj, z_length)
}
