use super::types::{
    AdjC, Adjacency, Neighbors, Point, Tour, TourSlice, Vert2dd, Verts, VertsC2, VertsC3, Yarn, Varr16,
};

pub fn from_const_adj_to_adj(graph: &AdjC) -> Adjacency {
    graph
        .iter()
        .map(|(node, neighbors)| (*node, neighbors.iter().cloned().collect::<Neighbors>()))
        .collect()
}

pub fn from_verts_to_vertsc(verts: &Verts) -> Vec<[Point; 2]> {
    verts.iter().map(|(_x, _y, _)| [*_x, *_y]).collect()
}

pub fn from_verts_to_vertsc16(verts: &Varr16) -> Vec<[i16; 3]> {
    verts.iter().map(|[_x, _y, _z]| [*_x, *_y, *_z]).collect()
}

pub fn from_nodes_to_yarn(path: Tour, verts: &Vert2dd) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

pub fn from_v3c_to_v2c(verts: &VertsC3) -> Vert2dd {
    verts.iter().clone().map(|&(x, y, _)| (x, y)).collect()
}

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

pub fn convert_to_varr16(verts: &Verts) -> Varr16 {
    verts.iter().map(|(x, y, z)| [*x as i16, *y as i16, *z as i16]).collect()
}