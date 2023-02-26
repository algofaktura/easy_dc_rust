use std::collections::{HashMap, HashSet};

pub fn make_weights(adj: &HashMap<u32, HashSet<u32>> , verts: &[(i32, i32, i32)]) -> HashMap<u32, i32> {
    adj.iter().map(|(&n, _)| {
        let (x, y, z): (i32, i32, i32) = verts[n as usize];
        let weight: i32 = x.abs() + y.abs() + z.abs();
        (n, weight)
    }).collect()
}

pub fn make_weights_ref<'a>(adj: &'a HashMap<&u32, HashSet<&u32>>, verts: &[(i32, i32, i32)]) -> HashMap<&'a u32, i32> {
    adj.iter().map(|(&n, _)| {
        let (x, y, z): (i32, i32, i32) = verts[*n as usize];
        let weight: i32 = x.abs() + y.abs() + z.abs();
        (n, weight)
    }).collect()
}

