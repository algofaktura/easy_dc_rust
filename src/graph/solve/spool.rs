extern crate itertools;

use itertools::Itertools;

use crate::graph::operators::{color, get_axis};
use crate::graph::translate;
use crate::graph::types::{
    Adjacency, Bobbins, Count, Idx, Loom, Node, Spool, Subtours, Tour, TourSlice, V3Slice, V3d,
    VIMap, Varr, Vert, Vert2dd, Verts, Weights, Yarn,
};

pub fn yarn(z_adj: &Adjacency, verts: &Verts, var: &Varr, weights: &Weights) -> Spool {
    let verts2dd: &Vert2dd = &translate::from_v3c_to_v2c(verts);
    let path: Tour = spin(&z_adj, &weights, var);
    let natural: Yarn = translate::from_nodes_to_yarn(path, verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn spin(adj: &Adjacency, weights: &Weights, verts: V3Slice) -> Tour {
    let path: &mut Tour = &mut vec![*adj.keys().max().unwrap() as Node];
    let order: Count = adj.len();
    for idx in 1..order {
        path.push(if idx < order - 5 {
            get_next(&path, adj, weights)
        } else {
            get_next_xyz(&path, adj, weights, verts)
        })
    }
    path.to_vec()
}

pub fn get_next(path: TourSlice, adj: &Adjacency, weights: &Weights) -> Node {
    adj.get(path.last().unwrap())
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .copied()
        .max_by_key(|&n| *weights.get(&n).unwrap())
        .unwrap()
}

pub fn get_next_xyz(path: TourSlice, adj: &Adjacency, weights: &Weights, verts: V3Slice) -> Node {
    let curr: &Node = path.last().unwrap();
    let curr_vert: &V3d = &verts[*curr as usize];
    adj.get(curr)
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .map(|&n| (n, get_axis(curr_vert, &verts[n as usize])))
        .filter(|(_, next_axis)| {
            *next_axis != get_axis(&verts[path[path.len() - 2] as usize], curr_vert)
        })
        .max_by_key(|&(n, _)| weights[&n])
        .unwrap()
        .0
}

pub fn wind(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) -> Bobbins {
    loom.iter_mut()
        .map(|thread| {
            let (left, right) = get_upper_nodes(
                verts[thread[0] as usize],
                verts[thread[thread.len() - 1] as usize],
                vert_idx,
            );
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .flatten()
        .collect()
}

pub fn get_upper_nodes((x, y, z): Vert, (x1, y1, z1): Vert, vert_idx: &VIMap) -> (u32, u32) {
    (vert_idx[&(x, y, z + 2)], vert_idx[&(x1, y1, z1 + 2)])
}

pub fn cut(tour: Tour, subset: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: Idx = tour.len() - 1;
    let last_idx: Idx = subset.len() - 1;
    let mut prev: i32 = -1 as i32;
    for (e, idx) in tour
        .iter()
        .enumerate()
        .filter_map(|(i, &node)| {
            if subset.contains(&node) {
                Some(i)
            } else {
                None
            }
        })
        .sorted()
        .enumerate()
    {
        if e == last_idx && idx != last_ix {
            for subtour in vec![
                tour[(prev + 1) as usize..idx].to_vec(),
                tour[idx..].to_vec(),
            ] {
                if !subtour.is_empty() {
                    subtours.push(if subset.contains(&subtour[0]) {
                        subtour
                    } else {
                        subtour.iter().rev().cloned().collect()
                    });
                }
            }
        } else {
            let subtour = tour[(prev + 1) as usize..=idx].to_vec();
            if !subtour.is_empty() {
                subtours.push(if subset.contains(&subtour[0]) {
                    subtour
                } else {
                    subtour.iter().rev().cloned().collect()
                });
            }
            prev = idx as i32;
        }
    }
    subtours
}
