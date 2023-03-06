use crate::types::types::*;

pub fn make_weights(adj: &Adjacency, verts: &VertsC3) -> Weights {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): Vert3d = verts[n as usize];
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

fn get_adj_edges(adj: &Adjacency, m_node: Node, n_node: Node) -> Edges {
    adj.get(&m_node).unwrap()
       .iter()
       .flat_map(|m| adj.get(&n_node).unwrap().iter().map(move |n| (*m, *n)))
       .filter(|(m, n)| adj.get(m).unwrap().contains(n))
       .map(|(m, n)| {
            if m < n { (m, n) } else { (n, m) }
        })
        .collect()
}

pub fn make_edges_adj(adj: &Adjacency, edges: &Edges) -> EdgeAdjacency {
    edges
        .iter()
        .map(|&(m, n)| ((m, n), get_adj_edges(adj, m, n)))
        .collect()
}

pub fn make_edges_adj1(a: &Adjacency, edges: &Edges) -> EdgeAdjacency {
    edges
        .iter()
        .map(|&(u, p)| {
            let lhs = a
                .get(&u)
                .unwrap()
                .difference(&Neighbors::from([p]))
                .cloned()
                .collect::<Neighbors>();
            let rhs = a
                .get(&p)
                .unwrap()
                .difference(&Neighbors::from([u]))
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