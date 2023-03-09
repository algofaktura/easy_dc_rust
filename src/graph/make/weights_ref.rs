use std::collections::HashMap;

use super::{super::types::{Adjacency, Vert3d, VertsC3, Weights}, vert_idx::VertRef};

pub fn make_weights_ref<'a>(vert_idx: &HashMap<(i32, i32, i32), &'a (i32, i32, i32)>) -> HashMap<VertRef<'a>, i32> {
    vert_idx
        .keys()
        .cloned()
        .map(|v| {
            (*vert_idx.get(&v).unwrap(), v.0.abs() + v.1.abs() + v.2.abs())
        })
        .collect::<HashMap<VertRef, i32>>()
}

pub fn make_weightsvec(adj: &Adjacency, verts: &VertsC3) -> Weights {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): Vert3d = verts[n as usize];
            let weight: i32 = x.abs() + y.abs() + z.abs();
            (n, weight)
        })
        .collect()
}
