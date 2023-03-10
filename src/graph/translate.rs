use super::types::{
    // AdjC, Adjacency, Neighbors, Point, Tour, TourSlice, Vectors2d, Vectors3d, Vert2dd, Verts,
    AdjC, Adjacency, Neighbors, Point, Tour, TourSlice, Vert2dd, Verts,
    VertsC2, VertsC3, Yarn,
};
// use crate::structs::vector::{Vector2D, Vector3D};

pub fn from_const_adj_to_adj(graph: &AdjC) -> Adjacency {
    graph
        .iter()
        .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect::<Neighbors>()))
        .collect()
}

// pub fn from_v3c_to_vect3d(verts: &VertsC3) -> Vectors3d {
//     verts
//         .iter()
//         .map(|(_x, _y, _z)| Vector3D {
//             x: *_x,
//             y: *_y,
//             z: *_z,
//         })
//         .collect::<Vectors3d>()
// }

pub fn from_verts_to_vertsc(verts: &Verts) -> Vec<[Point; 3]> {
    verts
        .iter()
        .map(|(_x, _y, _z)| [*_x, *_y, *_z])
        .collect::<Vec<[Point; 3]>>()
}

pub fn from_nodes_to_yarn(path: Tour, verts: &Vert2dd) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

// pub fn from_v3c_to_vect2d(verts: &VertsC3) -> Vectors2d {
//     verts
//         .iter()
//         .map(|v| Vector2D { x: v.0, y: v.1 })
//         .collect::<Vectors2d>()
// }

pub fn from_v3c_to_v2c(verts: &VertsC3) -> Vert2dd {
    verts.iter().clone().map(|&(x, y, _)| (x, y)).collect()
}

// pub fn convert_to_2d(vec3ds: &Vectors3d) -> Vectors2d {
//     vec3ds.iter().map(|v| v.to_2d()).collect()
// }

// pub fn convert_from_3d(vec3ds: &Vectors3d) -> Vectors2d {
//     vec3ds.iter().map(|v| Vector2D::from_3d(*v)).collect()
// }

pub fn convert_from_nodes(path: Tour, verts: &Vert2dd) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

pub fn convert_from_nodes_slice(path: TourSlice, verts: &VertsC2) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

pub fn convert_from_nodes_general<T>(path: &[T], verts: &VertsC2) -> Yarn
where
    T: TryInto<usize> + Copy,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Yarn::from(
        path.iter()
            .map(|&n| {
                let vector = verts[n.try_into().unwrap()];
                [vector.0, vector.1]
            })
            .collect::<Vec<[Point; 2]>>(),
    )
}
