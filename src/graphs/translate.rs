use std::collections::{HashMap, HashSet};
use crate::structs::vector3d::Vector3D;

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

pub fn translate_verts(verts: &[(i32, i32, i32)]) -> Vec<Vector3D> {
    verts.iter().map(|v| Vector3D { x: v.0, y: v.1, z: v.2, }).collect()
}

pub fn make_vi_mapping(verts: &Vec<Vector3D>) -> HashMap<&Vector3D, u32> {
    verts.iter()
         .enumerate()
         .map(|(idx, vert)| {(vert, idx as u32)}).collect()
}