use itertools::Itertools;

use std::collections::{HashMap, HashSet};

use super::{super::graph::types::{Count, Point, Points}, types::{Verts, AdjacencyRef}, make::vert_idx::VertRef};

type ZlevelMapRef<'a> = HashMap<i32, HashSet<VertRef<'a>>>;

pub fn shrink_adjacency_ref<'a>(verts: &'a Verts, adj: &'a AdjacencyRef) -> (AdjacencyRef<'a>, Vec<(i32, usize)>) {
    let stratified: ZlevelMapRef = stratified_nodes_ref(verts);
    let z_adj: AdjacencyRef = filter_graph_ref(&adj, stratified[&(-1 as i32)].clone());
    let z_length = get_zlevel_length_ref(&stratified);
    (z_adj, z_length)
}

fn stratified_nodes_ref(verts: &Verts) -> ZlevelMapRef {
    verts
        .iter()
        .map(|v| v.2)
        .filter(|&z| z < 0i32)
        .collect::<Points>()
        .into_iter()
        .map(|z| {
            let nodes = verts
                .iter()
                .filter(|v| v.2 as i32 == z )
                .collect::<HashSet<VertRef>>();
            (z, nodes)
        })
        .collect()
}

fn filter_graph_ref<'a>(adj: &'a AdjacencyRef, vert_refs: HashSet<&'a (i32, i32, i32)>) -> AdjacencyRef<'a> {
    adj.iter()
        .filter(|(k, _)| vert_refs.contains(*k))
        .map(|(k, v)| (*k, v.intersection(&vert_refs).copied().collect()))
        .collect()
}

pub fn get_zlevel_length_ref(stratified: &ZlevelMapRef) -> Vec<(Point, Count)> {
    stratified
        .iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .sorted_by_key(|&(level, _)| level)
        .collect::<Vec<(Point, Count)>>()
}
