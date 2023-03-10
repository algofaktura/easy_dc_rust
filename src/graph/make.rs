use itertools::Itertools;
use ndarray::arr2;

use crate::utils::operators::{absumv, edist};
use crate::utils::operators::shift_xyz;

use crate::graph::types::{Adjacency, Edges, EdgeAdjacency, Idx, Node, Nodes, Point, VIMap, Verts, VertsC3, Weights, Vert};
use crate::graph::check::is_valid_edge;

pub fn make_vertices(max_xyz: Point) -> Verts {
    (-(max_xyz)..=(max_xyz))
        .step_by(2)
        .flat_map(|x| {
            (-max_xyz..=max_xyz)
                .step_by(2)
                .flat_map(move |y| {
                    (-max_xyz..=max_xyz)
                        .step_by(2)
                        .map(move |z| (x, y, z))
                        .filter(|&v| absumv(v) < (max_xyz + 4))
                        .collect::<Verts>()
                })
                .collect::<Verts>()
        })
        .into_iter()
        .sorted_by_key(|v| (edist(*v), v.0, v.1, v.2))
        .collect::<Verts>()
}

pub fn make_vi_mapping(verts: &Verts) -> VIMap {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as Node))
        .collect::<VIMap>()
}

pub fn make_adj(verts: &VertsC3, max_xyz: Point, vi: &VIMap) -> Adjacency {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| {
            (
                idx as Node,
                shift_xyz(arr2(&[[vert.0, vert.1, vert.2]]))
                    .outer_iter()
                    .filter(|new_vert| {
                        absumv((new_vert[0], new_vert[1], new_vert[2])) <= max_xyz + 2
                    })
                    .map(|new_vert| *vi.get(&(new_vert[0], new_vert[1], new_vert[2])).unwrap())
                    .filter(|&m| m != (idx as Node))
                    .collect::<Nodes>(),
            )
        })
        .collect::<Adjacency>()
}

pub fn make_edges_from_adj(adj: &Adjacency) -> Edges {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .collect()
}

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

pub fn make_weights(adj: &Adjacency, verts: &Verts) -> Weights {
    adj.iter()
        .map(|(&n, _)| {
            let (x, y, z): Vert = verts[n as usize];
            (n, x.abs() + y.abs() + z.abs())
        })
        .collect()
}
