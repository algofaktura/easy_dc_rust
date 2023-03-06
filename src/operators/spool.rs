extern crate itertools;

use itertools::Itertools;

use super::operators::{color, get_axis};

use crate::{
    graphs::utils::{
        make::make_weights,
        map::{convert_from_nodes, make_verts2dd},
    },
    types::types::{
        Adjacency, Bobbins, Count, Idx, Loom, Node, Spool, Subtours, Tour, TourSlice, V3Slice, V3d,
        Vectors3d, Vert2dd, VertIdx, VertsC3, Weights, Yarn,
    },
};

pub fn spool_yarn(z_adj: &Adjacency, verts: &VertsC3, var: &[[i32; 3]]) -> Spool {
    let verts2dd: &Vert2dd = &make_verts2dd(verts);
    let weights: Weights = make_weights(z_adj, verts);
    let path: Tour = spin(&z_adj, &weights, var);
    let natural: Yarn = convert_from_nodes(path, verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn wind(loom: &mut Loom, verts: &Vectors3d, vert_idx: &VertIdx) -> Bobbins {
    loom.iter_mut()
        .map(|thread| {
            let left = verts[thread[0] as usize].get_upper_node(&vert_idx);
            let right = verts[thread[thread.len() - 1] as usize].get_upper_node(&vert_idx);
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .flatten()
        .collect::<Bobbins>()
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

pub fn spin(adj: &Adjacency, weights: &Weights, verts: V3Slice) -> Tour {
    let path: &mut Tour = &mut vec![*adj.keys().max().unwrap() as Node];
    let order: Count = adj.len();
    let limit: Count = order - 5;
    for idx in 1..order {
        path.push(if idx < limit {
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
    let prev_vert: &V3d = &verts[path[path.len() - 2] as usize];
    adj.get(curr)
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .map(|&n| (n, get_axis(curr_vert, &verts[n as usize])))
        .filter(|(_, next_axis)| *next_axis != get_axis(prev_vert, curr_vert))
        .max_by_key(|&(n, _)| weights[&n])
        .unwrap()
        .0
}
