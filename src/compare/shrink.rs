use itertools::Itertools;

use super::super::graph::types::{Adjacency, Nodes, Point, Points, Vectors3d, Verts, ZlevelNodesMap, ZOrder};

pub fn shrink_adjacency(vects3d: &Vectors3d, adj: &Adjacency) -> (Adjacency, ZOrder) {
    let stratified: ZlevelNodesMap = stratified_nodes_v3d(vects3d);
    (
        filtered_adjacency(&adj, stratified[&(-1 as Point)].clone()), 
        get_zlevel_length(&stratified)
    )
}

fn stratified_nodes(verts: &Verts) -> ZlevelNodesMap {
    verts
        .iter()
        .map(|v| v.2)
        .filter(|&z| z < 0)
        .collect::<Points>()
        .into_iter()
        .map(|z| {
            let nodes = verts
                .iter()
                .enumerate()
                .filter(|&(_, v)| v.2 as Point == z)
                .map(|(i, _)| i as u32)
                .collect::<Nodes>();
            (z, nodes)
        })
        .collect()
}

fn filtered_adjacency(adj: &Adjacency, nodes: Nodes) -> Adjacency {
    adj.iter()
        .filter(|(k, _)| nodes.contains(k))
        .map(|(k, v)| (*k, v.intersection(&nodes).copied().collect()))
        .collect()
}

pub fn get_zlevel_length(stratified: &ZlevelNodesMap) -> ZOrder {
    stratified
        .iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .sorted_by_key(|&(level, _)| level)
        .collect::<ZOrder>()
}

pub fn shrink_adjacency_2(verts: &Verts, adj: &Adjacency) -> (Adjacency, ZOrder) {
    let stratified: ZlevelNodesMap = stratified_nodes(verts);
    (
        filtered_adjacency(&adj, stratified[&(-1 as Point)].clone()), 
        get_zlevel_length(&stratified)
    )
}

fn stratified_nodes_v3d(vects3d: &Vectors3d) -> ZlevelNodesMap {
    vects3d
        .iter()
        .map(|v| v.z)
        .filter(|&z| z < 0)
        .collect::<Points>()
        .into_iter()
        .map(|z| {
            let nodes = vects3d
                .iter()
                .enumerate()
                .filter(|&(_, v)| v.z as Point == z)
                .map(|(i, _)| i as u32)
                .collect::<Nodes>();
            (z, nodes)
        })
        .collect()
}
