use itertools::Itertools;

use super::types::{Adjacency, Nodes, Point, Points, Verts, ZOrder, ZlevelNodesMap};

pub fn adjacency(verts: &Verts, adj: &Adjacency) -> (Adjacency, ZOrder) {
    let stratified: ZlevelNodesMap = stratify_nodes(verts);
    (
        filter_adjacency(&adj, stratified[&(-1 as Point)].clone()),
        get_zlevel_order(&stratified),
    )
}

fn stratify_nodes(verts: &Verts) -> ZlevelNodesMap {
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

fn filter_adjacency(adj: &Adjacency, nodes: Nodes) -> Adjacency {
    adj.iter()
        .filter(|(k, _)| nodes.contains(k))
        .map(|(k, v)| (*k, v.intersection(&nodes).copied().collect()))
        .collect()
}

pub fn get_zlevel_order(stratified: &ZlevelNodesMap) -> ZOrder {
    stratified
        .iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .sorted_by_key(|&(level, _)| level)
        .collect()
}
