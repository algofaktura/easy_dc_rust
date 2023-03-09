use crate::graph::types::{Verts, Weightsi16};

use super::super::types::{Adjacency, Vert3d, Weights, Verti16};

pub fn make_weights(adj: &Adjacency, verts: &Verts) -> Weights {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): Vert3d = verts[n as usize];
            (n, x.abs() + y.abs() + z.abs())
        })
        .collect()
}
pub fn make_weights_i16(adj: &Adjacency, verts: &Vec<Verti16>) -> Weightsi16 {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): Verti16 = verts[n as usize];
            (n, (x.abs() + y.abs() + z.abs()) as i16)
        })
        .collect()
}
