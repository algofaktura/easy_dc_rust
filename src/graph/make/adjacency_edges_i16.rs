use itertools::Itertools;
use ndarray::arr2;
use std::collections::{HashMap, HashSet};

use crate::graph::types::{Edges, Verti16};

use super::super::operators::shift_xyz_i16;
use super::super::measure::absumv_i16;

pub fn get_adj_i16(
    verts: &[Verti16],
    max_xyz: i32,
    vi: &HashMap<Verti16, u32>,
) -> HashMap<u32, HashSet<u32>> {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| {
            (
                idx as u32,
                shift_xyz_i16(arr2(&[[vert.0, vert.1, vert.2]]))
                    .outer_iter()
                    // use filter map here
                    .filter(|new_vert| {
                        absumv_i16((new_vert[0], new_vert[1], new_vert[2])) <= max_xyz as u32 + 2
                    })
                    .map(|new_vert| *vi.get(&(new_vert[0], new_vert[1], new_vert[2])).unwrap())
                    .filter(|&m| m != (idx as u32))
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>()
}

pub fn get_edges_i16(adj: &HashMap<u32, HashSet<u32>>) -> HashSet<(u32, u32)> {
    adj.iter()
        .flat_map(|(k, v)| v.iter().map(move |&i| (*k, i)))
        .collect()
}

pub fn make_edges_i16(vertices: &[Verti16]) -> Edges {
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
