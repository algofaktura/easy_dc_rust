use crate::structs::vector::{Vector2D, Vector3D};
use crate::types::types::*;

pub fn map_graph(graph: &AdjC) -> Adjacency {
    graph
        .iter()
        .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect::<Neighbors>()))
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

pub fn vectorize2(verts: &VertsC3) -> Vectors3d {
    verts
        .iter()
        .map(|(_x, _y, _z)| Vector3D {
            x: *_x,
            y: *_y,
            z: *_z,
        })
        .collect::<Vectors3d>()
}

pub fn convert_from_nodes(path: Tour, verts: &Vert2dd) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i32; 2]>>(),
    )
}

pub fn translate_verts_2d(verts: &VertsC3) -> Vectors2d {
    verts
        .iter()
        .map(|v| Vector2D { x: v.0, y: v.1 })
        .collect::<Vectors2d>()
}

pub fn make_verts2dd(verts: &VertsC3) -> Vert2dd {
    verts.iter().clone().map(|&(x, y, _)| (x, y)).collect()
}
