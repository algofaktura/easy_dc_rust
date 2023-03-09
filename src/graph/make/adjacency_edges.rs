use itertools::Itertools;
use ndarray::arr2;
use std::collections::{HashMap, HashSet};

use crate::graph::types::Edges;

use super::super::operators::shift_xyz;
use super::super::measure::absumv;

pub fn get_adj(
    verts: &[(i32, i32, i32)],
    max_xyz: i32,
    vi: &HashMap<(i32, i32, i32), u32>,
) -> HashMap<u32, HashSet<u32>> {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| {
            (
                idx as u32,
                shift_xyz(arr2(&[[vert.0, vert.1, vert.2]]))
                    .outer_iter()
                    .filter(|new_vert| {
                        absumv((new_vert[0], new_vert[1], new_vert[2])) <= max_xyz as u32 + 2
                    })
                    .map(|new_vert| *vi.get(&(new_vert[0], new_vert[1], new_vert[2])).unwrap())
                    .filter(|&m| m != (idx as u32))
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}

pub fn get_edges(adj: &HashMap<u32, HashSet<u32>>) -> HashSet<(u32, u32)> {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .collect()
}

pub fn make_edges(vertices: &[(i32, i32, i32)]) -> Edges {
    vertices
        .iter()
        .enumerate()
        .combinations(2)
        .map(|pairs| (pairs[0], pairs[1]))
        .filter_map(|((i, &p), (j, &q))| {
            let (dx, dy, dz) = (p.0 - q.0, p.1 - q.1, p.2 - q.2);
            if (dx * dx + dy * dy + dz * dz) as u32 == 4 {
                Some(if i < j {
                    (i as u32, j as u32)
                } else {
                    (j as u32, i as u32)
                })
            } else {
                None
            }
        })
        .collect()
}
