use crate::types::types::{Adjacency, Nodes, Point, Points, Vectors3d, ZlevelNodesMap, Count};

pub fn shrink_adjacency(vects3d: &Vectors3d, adj: &Adjacency) -> (Adjacency, Vec<(i32, usize)>) {
    let stratified: ZlevelNodesMap = stratified_nodes(vects3d);
    let nodes: Nodes = stratified[&(-1 as i32)].clone();
    let z_adj: Adjacency = filter_graph(&adj, &nodes);
    let z_length = get_zlevel_length(&stratified);
    (z_adj, z_length)
}
fn stratified_nodes(vects3d: &Vectors3d) -> ZlevelNodesMap {
    vects3d
        .iter()
        .map(|v| v.z)
        .filter(|&z| z < 0i32)
        .collect::<Points>()
        .into_iter()
        .map(|z| {
            let nodes = vects3d
                .iter()
                .enumerate()
                .filter(|&(_, v)| v.z as i32 == z)
                .map(|(i, _)| i as u32)
                .collect::<Nodes>();
            (z, nodes)
        })
        .collect()
}

fn filter_graph(
    adj: &Adjacency,
    nodes: &Nodes,
) -> Adjacency {
    let filtered: Adjacency = adj
        .iter()
        .filter(|(k, _)| nodes.contains(k))
        .map(|(k, v)| (*k, v.intersection(nodes).copied().collect()))
        .collect();
    filtered
}

pub fn get_zlevel_length(stratified: &ZlevelNodesMap) -> Vec<(Point, Count)> {
    let mut vec = stratified
        .iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .collect::<Vec<(Point, Count)>>();
    vec.sort_by_key(|&(level, _)| level);
    vec
}
