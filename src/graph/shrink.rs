use std::collections::HashSet;

use itertools::Itertools;

use super::types::{Adjacency, Nodes, Point, Points, Verts, ZOrder, ZlevelNodesMap, Varr, ZlevelNodesMap16, ZOrderVar};

pub fn adjacency(verts: &Verts, adj: &Adjacency) -> (Adjacency, ZOrder) {
    let stratified: ZlevelNodesMap = stratify_nodes(verts);
    (
        filter_adjacency(&adj, stratified[&(-1 as Point)].clone()),
        get_zlevel_order(&stratified),
    )
}

pub fn adjacency_var(verts: &Varr, adj: &Adjacency) -> (Adjacency, ZOrderVar) {
    let stratified: ZlevelNodesMap16 = stratify_nodes_var(verts);
    (
        filter_adjacency(&adj, stratified[&(-1 as i16)].clone()),
        get_zlevel_order_var(&stratified),
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

fn stratify_nodes_var(verts: &Varr) -> ZlevelNodesMap16 {
    verts
        .iter()
        .map(|v| v[2])
        .filter(|&z| z < 0)
        .collect::<HashSet<i16>>()
        .into_iter()
        .map(|z| {
            let nodes = verts
                .iter()
                .enumerate()
                .filter(|&(_, v)| v[2] as i16 == z)
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

pub fn get_zlevel_order_var(stratified: &ZlevelNodesMap16) -> Vec<(i16, usize)> {
    stratified
        .iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .sorted_by_key(|&(level, _)| level)
        .collect()
}
