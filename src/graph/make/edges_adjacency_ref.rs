use std::collections::{HashSet, HashMap};

use crate::graph::types::{Adjacency, EdgeAdjacency, Edges, Node, VertsC3, Verts};
use crate::graph::check::is_valid_edge;
use crate::structs::edge::Edge;

pub fn make_edges_adj_const(adj: &Adjacency, edges: &Edges, verts: &VertsC3) -> EdgeAdjacency {
    edges
        .iter()
        .filter(|&(a, b)| is_valid_edge(verts[*a as usize], verts[*b as usize]))
        .map(|&(m, n)| ((m, n), get_adj_edges(adj, m, n, verts)))
        .collect()
}

pub fn make_edges_adj_ref<'a>(
    adj: &'a HashMap<&'a Verti16, HashSet<&'a Verti16>>,
    edges: &'a HashSet<(&'a Verti16, &'a Verti16)>,
) -> HashMap<(&'a Verti16, &'a Verti16), HashSet<(&'a Verti16, &'a Verti16)>> {
    edges
        .iter()
        .filter(|&(a, b)| is_valid_edge_ref(*a, *b))
        .map(|&(m, n)| ((m, n), get_adj_edges_ref(adj, m, n)))
        .collect()
}

pub fn get_adj_edges_ref<'a>(adj: &'a HashMap<&'a Verti16, HashSet<&'a Verti16>>, m_node: &'a Verti16, n_node: &'a Verti16) -> HashSet<(&'a Verti16, &'a Verti16)> {
    adj.get(&m_node)
        .unwrap()
        .iter()
        .flat_map(|m| adj.get(&n_node).unwrap().iter().map(move |n| (*m, *n)))
        .filter(|(m, n)| {
            adj.get(*m).unwrap().contains(*n) && is_valid_edge_ref(*m, *n)
        })
        .map(|(m, n)| if m < n { (m, n) } else { (n, m) })
        .collect()
}

pub fn make_edges_adj_ref2<'a>(
    adj: &'a HashMap<&'a Verti16, HashSet<&'a Verti16>>,
    edges: &'a HashSet<(&'a Verti16, &'a Verti16)>,
) -> HashMap<&'a (&'a Verti16, &'a Verti16), HashSet<&'a (&'a Verti16, &'a Verti16)>> {
    edges
        .iter()
        .filter(|edge| is_valid_edge_ref(edge.0, edge.1))
        .map(|edge| (edge, get_adj_edges_ref2(adj, edge.0, edge.1)))
        .collect::<HashMap<&'a (&'a Verti16, &'a Verti16), HashSet<&'a (&'a Verti16, &'a Verti16)>>>()
}

pub fn make_edge_set<'a>(
    edges: &'a HashSet<(&'a Verti16, &'a Verti16)>,
) -> HashSet<(&'a Verti16, &'a Verti16)> {
    edges.iter().map(|(m, n)| (*m, *n)).collect()
}

pub fn get_adj_edges_ref2<'a>(
    adj: &'a HashMap<&'a Verti16, HashSet<&'a Verti16>>,
    m_node: &'a Verti16,
    n_node: &'a Verti16,
) -> HashSet<&'a (&'a Verti16, &'a Verti16)> {
    adj.get(m_node)
        .unwrap()
        .iter()
        .flat_map(|&m| {
            adj.get(n_node)
                .unwrap()
                .iter()
                .filter(move |&&n| {
                    adj.get(n).unwrap().contains(m) && is_valid_edge_ref(m, n)
                })
                .map(move |&n| (m, n))
        })
        .map(|(m, n)| if m < n { &(m, n) } else { &(n, m) })
        .collect::<HashSet<_>>()
}

pub fn is_valid_edge_ref((x1, y1, _): &Verti16, (x2, y2, _): &Verti16) -> bool {
    let total = (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF);
    (4 <= total) && (total <= 10)
}
