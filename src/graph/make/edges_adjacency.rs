use crate::graph::types::{Adjacency, EdgeAdjacency, Edges, Node, Verts, Idx};
use crate::graph::check::is_valid_edge;

pub fn make_edges_adj(adj: &Adjacency, edges: &Edges, verts: &Verts) -> EdgeAdjacency {
    edges
        .iter()
        .filter(|&(a, b)| is_valid_edge(verts[*a as Idx], verts[*b as Idx]))
        .map(|&(m, n)| ((m, n), get_adj_edges(adj, m, n, verts)))
        .collect()
}

pub fn get_adj_edges(adj: &Adjacency, m_node: Node, n_node: Node, verts: &Verts) -> Edges {
    adj.get(&m_node)
        .unwrap()
        .iter()
        .flat_map(|m| adj.get(&n_node).unwrap().iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| {
            adj.get(m).unwrap().contains(n) && is_valid_edge(verts[*m as Idx], verts[*n as Idx])
        })
        .map(|(m, n)| if m < n { (m, n) } else { (n, m) })
        .collect()
}
