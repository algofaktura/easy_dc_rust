use std::collections::HashSet;

use crate::types::types::*;

pub fn make_weights(adj: &Adjacency, verts: &VertsC3) -> Weights {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): VertsTriple = verts[n as usize];
            let weight: i32 = x.abs() + y.abs() + z.abs();
            (n, weight)
        })
        .collect()
}
pub fn make_vi_mapping(verts: &Vectors3d) -> VertIdx {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (vert, idx as u32))
        .collect::<VertIdx>()
}

pub fn make_edges_adj(a: &Adjacency, edges: &Edges) -> EdgeAdjacency {
    edges
        .iter()
        .map(|&(u, p)| {
            let lhs = a
                .get(&u)
                .unwrap()
                .difference(&HashSet::from([p]))
                .cloned()
                .collect::<Neighbors>();
            let rhs = a
                .get(&p)
                .unwrap()
                .difference(&HashSet::from([u]))
                .cloned()
                .collect::<Neighbors>();
            let prod = lhs
                .iter()
                .flat_map(|&m| {
                    rhs.iter()
                        .map(move |&n| if m < n { (m, n) } else { (n, m) })
                })
                .collect::<Edges>();
            ((u, p), prod.intersection(edges).cloned().collect::<Edges>())
        })
        .filter(|(_, s)| !s.is_empty())
        .collect()
}
