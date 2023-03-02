use std::collections::{HashMap, HashSet};

use ndarray::Array2;

use crate::structs::vector::Vector3D;

pub fn map_graph(graph: &[(u32, &[u32])]) -> HashMap<u32, HashSet<u32>> {
    graph.iter()
         .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect()))
         .collect()
}

pub fn vectorize(verts: &[(i32, i32, i32)]) -> Vec<Vector3D> {
    verts.iter()
         .map(|v| Vector3D { x: v.0, y: v.1, z: v.2, })
         .collect::<Vec<Vector3D>>()
}

pub fn convert_from_nodes(path: Vec<u32>, verts: &Vec<(i32, i32)>) -> Array2<i32> {
    Array2::from(path.iter().map(|&n| [verts[n as usize].0, verts[n as usize].1]).collect::<Vec<[i32; 2]>>())
}
