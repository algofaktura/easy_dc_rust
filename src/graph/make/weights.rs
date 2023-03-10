use super::super::types::{Adjacency, Vert3d, Verts, Weights};

pub fn make_weights(adj: &Adjacency, verts: &Verts) -> Weights {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): Vert3d = verts[n as usize];
            (n, x.abs() + y.abs() + z.abs())
        })
        .collect()
}
