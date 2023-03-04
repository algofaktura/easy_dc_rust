use ndarray::Array2;

use crate::structs::vector::{Vector2D, Vector3D};
use crate::types::types::*;

pub fn map_graph(graph: &AdjC) -> Adjacency {
    graph
        .iter()
        .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect()))
        .collect()
}

pub fn vectorize(verts: &VertsC3) -> Vectors3d {
    verts
        .iter()
        .map(|v| Vector3D {
            x: v.0,
            y: v.1,
            z: v.2,
        })
        .collect::<Vectors3d>()
}

pub fn convert_from_nodes(path: Path, verts: &Vert2dd) -> Yarn {
    Array2::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i32; 2]>>(),
    )
}

pub fn translate_verts_2d(verts: &[(i32, i32, i32)]) -> Vec<Vector2D> {
    verts
        .iter()
        .map(|v| Vector2D { x: v.0, y: v.1 })
        .collect::<Vec<Vector2D>>()
}
