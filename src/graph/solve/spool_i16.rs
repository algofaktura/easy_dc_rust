extern crate itertools;

use std::collections::HashMap;

use itertools::Itertools;

use crate::graph::operators::{color_i16, get_axis_i16};
use crate::graph::translate;
use crate::graph::types::{
    Adjacency, Bobbins, Count, Idx, Loom, Node, Spooli16, Subtours, Tour, TourSlice, V3Slicei16, V3di16,
    Vertsi16, Weightsi16, Yarni16, Vert2ddi16, Verti16,
};
use crate::structs::vector_i16::Vectors3di16;

pub fn yarn(z_adj: &Adjacency, verts: &Vertsi16, var: &[[i16; 3]], weights: &Weightsi16) -> Spooli16 {
    let verts2dd: &Vert2ddi16 = &translate::from_v3c_to_v2c_i16(verts);
    let path: Tour = spin(&z_adj, &weights, var);
    let natural: Yarni16 = translate::from_nodes_to_yarn_i16(path, verts2dd);
    let colored: Yarni16 = color_i16(&natural);
    Spooli16::from([(3, natural), (1, colored)])
}

pub fn spin(adj: &Adjacency, weights: &Weightsi16, verts: V3Slicei16) -> Tour {
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

pub fn get_next(path: TourSlice, adj: &Adjacency, weights: &Weightsi16) -> Node {
    adj.get(path.last().unwrap())
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .copied()
        .max_by_key(|&n| *weights.get(&n).unwrap())
        .unwrap()
}

pub fn get_next_xyz(path: TourSlice, adj: &Adjacency, weights: &Weightsi16, verts: V3Slicei16) -> Node {
    let curr: &Node = path.last().unwrap();
    let curr_vert: &V3di16 = &verts[*curr as usize];
    adj.get(curr)
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .map(|&n| (n, get_axis_i16(curr_vert, &verts[n as usize])))
        .filter(|(_, next_axis)| {
            *next_axis != get_axis_i16(&verts[path[path.len() - 2] as usize], curr_vert)
        })
        .max_by_key(|&(n, _)| weights[&n])
        .unwrap()
        .0
}

pub fn wind(loom: &mut Loom, verts: &Vectors3di16, vert_idx: &HashMap<Verti16, u32>) -> Bobbins {
    loom.iter_mut()
        .map(|thread| {
            let left = verts[thread[0] as usize].get_upper_node(&vert_idx);
            let right = verts[thread[thread.len() - 1] as usize].get_upper_node(&vert_idx);
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .flatten()
        .collect()
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
