use super::{super::graph::types::{Adjacency, Count, Nodes, Point, Points, Vectors3d, ZlevelNodesMap}, types::{Verts, Vertsi16, Pointsi16, ZlevelNodesMapi16, Pointi16}};

pub fn shrink_adjacency(vects3d: &Vectors3d, adj: &Adjacency) -> (Adjacency, Vec<(i32, usize)>) {
    let stratified: ZlevelNodesMap = stratified_nodes_v3d(vects3d);
    let nodes: Nodes = stratified[&(-1 as i32)].clone();
    let z_adj: Adjacency = filter_graph(&adj, &nodes);
    let z_length = get_zlevel_length(&stratified);
    (z_adj, z_length)
}

pub fn shrink_adjacency_2(verts: &Verts, adj: &Adjacency) -> (Adjacency, Vec<(i32, usize)>) {
    let stratified: ZlevelNodesMap = stratified_nodes(verts);
    let nodes: Nodes = stratified[&(-1 as i32)].clone();
    let z_adj: Adjacency = filter_graph(&adj, &nodes);
    let z_length = get_zlevel_length(&stratified);
    (z_adj, z_length)
}

pub fn shrink_adjacency_i16(verts: &Vertsi16, adj: &Adjacency) -> (Adjacency, Vec<(i16, usize)>) {
    let stratified: ZlevelNodesMapi16 = stratified_nodes_i16(verts);
    let nodes: Nodes = stratified[&(-1 as i16)].clone();
    let z_adj: Adjacency = filter_graph(&adj, &nodes);
    let z_length = get_zlevel_length_i16(&stratified);
    (z_adj, z_length)
}

fn stratified_nodes_v3d(vects3d: &Vectors3d) -> ZlevelNodesMap {
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

fn stratified_nodes(verts: &Verts) -> ZlevelNodesMap {
    verts
        .iter()
        .map(|v| v.2)
        .filter(|&z| z < 0i32)
        .collect::<Points>()
        .into_iter()
        .map(|z| {
            let nodes = verts
                .iter()
                .enumerate()
                .filter(|&(_, v)| v.2 as i32 == z)
                .map(|(i, _)| i as u32)
                .collect::<Nodes>();
            (z, nodes)
        })
        .collect()
}

fn stratified_nodes_i16(verts: &Vertsi16) -> ZlevelNodesMapi16 {
    verts
        .iter()
        .map(|v| v.2)
        .filter(|&z| z < 0i16)
        .collect::<Pointsi16>()
        .into_iter()
        .map(|z| {
            let nodes = verts
                .iter()
                .enumerate()
                .filter(|&(_, v)| v.2 as i16 == z)
                .map(|(i, _)| i as u32)
                .collect::<Nodes>();
            (z, nodes)
        })
        .collect()
}

fn filter_graph(adj: &Adjacency, nodes: &Nodes) -> Adjacency {
    adj.iter()
        .filter(|(k, _)| nodes.contains(k))
        .map(|(k, v)| (*k, v.intersection(nodes).copied().collect()))
        .collect()
}

pub fn get_zlevel_length(stratified: &ZlevelNodesMap) -> Vec<(Point, Count)> {
    let mut vec = stratified
        .iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .collect::<Vec<(Point, Count)>>();
    vec.sort_by_key(|&(level, _)| level);
    vec
}

pub fn get_zlevel_length_i16(stratified: &ZlevelNodesMapi16) -> Vec<(Pointi16, Count)> {
    let mut vec = stratified
        .iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .collect::<Vec<(Pointi16, Count)>>();
    vec.sort_by_key(|&(level, _)| level);
    vec
}
