extern crate itertools;

use itertools::Itertools;
use ndarray::arr2;

use super::spin::spin;

use crate::{
    graphs::utils::{make::make_weights, map::{convert_from_nodes, make_verts2dd}},
    types::types::{Adjacency, Bobbins, Idx, Loom, Spool, Subtours, Tour, Vectors3d, Vert2dd, VertsC3, VertIdx, Weights, Yarn},
};

pub fn spool_yarn(z_adj: &Adjacency, verts: &VertsC3, var: &[[i32; 3]]) -> Spool {
    let verts2dd: &Vert2dd = &make_verts2dd(verts);
    let weights: Weights = make_weights(z_adj, verts);
    let path: Tour = spin(&z_adj, &weights, var);
    let natural: Yarn = convert_from_nodes(path, verts2dd);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn reflect(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Yarn) -> Yarn {
    a + arr2(&[[0, 2]])
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
