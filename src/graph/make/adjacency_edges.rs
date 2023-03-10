use itertools::Itertools;
use ndarray::arr2;

use crate::graph::types::{Adjacency, Edges, Node, Nodes, Point, VIMap, VertsC3};

use super::super::operators::shift_xyz;
use super::super::measure::absumv;

pub fn make_adj(
    verts: &VertsC3,
    max_xyz: Point,
    vi: &VIMap,
) -> Adjacency {
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

pub fn make_edges1(vertices: &VertsC3) -> Edges {
    vertices
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pairs| (pairs[0], pairs[1]))
        .filter_map(|((i, &p), (j, &q))| {
            let (dx, dy, dz) = (p.0 - q.0, p.1 - q.1, p.2 - q.2);
            if (dx * dx + dy * dy + dz * dz) as Node == 4 {
                Some(if i < j {
                    (i as Node, j as Node)
                } else {
                    (j as Node, i as Node)
                })
            } else {
                None
            }
        })
        .collect()
}
