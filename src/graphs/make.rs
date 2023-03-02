use std::collections::{HashMap, HashSet};
use crate::structs::vector3d::Vector3D;

type AdjDict = HashMap<u32, HashSet<u32>>;
type Verts = Vec<Vector3D>;

pub fn make_weights(adj: &HashMap<u32, HashSet<u32>> , verts: &[(i32, i32, i32)]) -> HashMap<u32, i32> {
    adj.iter().map(|(&n, _)| {
        let (x, y, z): (i32, i32, i32) = verts[n as usize];
        let weight: i32 = x.abs() + y.abs() + z.abs();
        (n, weight)
    }).collect()
}
pub fn make_vi_mapping(verts: &Vec<Vector3D>) -> HashMap<&Vector3D, u32> {
    verts.iter()
         .enumerate()
         .map(|(idx, vert)| (vert, idx as u32)).collect::<HashMap<_, _>>()
}

pub fn make_edges_adj(a: &HashMap<u32, HashSet<u32>>, edges: &HashSet<(u32, u32)>) -> HashMap<(u32, u32), HashSet<(u32, u32)>> {
    edges.iter().map(|&(u, p)| {
        let lhs = a.get(&u).unwrap().difference(&HashSet::from([p])).cloned().collect::<HashSet<u32>>();
        let rhs = a.get(&p).unwrap().difference(&HashSet::from([u])).cloned().collect::<HashSet<u32>>();
        let prod = lhs.iter().flat_map(|&m| rhs.iter().map(move |&n| if m < n { (m, n) } else { (n, m) })).collect::<HashSet<(u32, u32)>>();

        ((u, p), prod.intersection(edges).cloned().collect::<HashSet<(u32, u32)>>())
    }).filter(|(_, s)| !s.is_empty()).collect()
}

pub fn graph_to_map(graph: &[(u32, &[u32])]) -> HashMap<u32, HashSet<u32>> {
    graph.iter()
         .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect()))
         .collect()
}

pub fn translate_verts_3d(verts: &[(i32, i32, i32)]) -> Vec<Vector3D> {
    verts.iter()
         .map(|v| Vector3D { x: v.0, y: v.1, z: v.2, })
         .collect::<Vec<Vector3D>>()
}

pub fn shrink_adjacency(vects3d: &Verts, adj: &AdjDict) -> (AdjDict, Vec<(i32, usize)> ) {
    let stratified = stratified_nodes(vects3d);
    let nodes: HashSet<u32> = stratified[&(-1 as i32)].clone();
    let z_adj: HashMap<u32, HashSet<u32>> = filter_graph(&adj, &nodes);
    let z_length = get_zlevel_length(&stratified);
    (z_adj, z_length)
}
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

pub fn get_zlevel_length(stratified: &HashMap<i32, HashSet<u32>>) -> Vec<(i32, usize)> {
    let mut vec = stratified.iter()
        .map(|(&level, nodes)| (level, nodes.len()))
        .collect::<Vec<_>>();
    vec.sort_by_key(|&(level, _)| level);
    vec
}