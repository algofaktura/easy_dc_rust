use std::collections::{HashMap, HashSet};

use crate::structs::vector::Vector3D;

pub fn make_weights(adj: &HashMap<u32, HashSet<u32>> , verts: &[(i32, i32, i32)]) -> HashMap<u32, i32> {
    adj.iter().map(|(&n, _)| {
        let (x, y, z): (i32, i32, i32) = verts[n as usize];
        let weight: i32 = x.abs() + y.abs() + z.abs();
        (n, weight)
    }).collect()
}
pub fn make_vi_mapping(verts: &Vec<Vector3D>) -> HashMap<&Vector3D, u32> {
    verts.iter()
         .enumerate()
         .map(|(idx, vert)| (vert, idx as u32)).collect::<HashMap<_, _>>()
}

pub fn make_edges_adj(a: &HashMap<u32, HashSet<u32>>, edges: &HashSet<(u32, u32)>) -> HashMap<(u32, u32), HashSet<(u32, u32)>> {
    edges.iter().map(|&(u, p)| {
        let lhs = a.get(&u).unwrap().difference(&HashSet::from([p])).cloned().collect::<HashSet<u32>>();
        let rhs = a.get(&p).unwrap().difference(&HashSet::from([u])).cloned().collect::<HashSet<u32>>();
        let prod = lhs.iter().flat_map(|&m| rhs.iter().map(move |&n| if m < n { (m, n) } else { (n, m) })).collect::<HashSet<(u32, u32)>>();

        ((u, p), prod.intersection(edges).cloned().collect::<HashSet<(u32, u32)>>())
    }).filter(|(_, s)| !s.is_empty()).collect()
}
