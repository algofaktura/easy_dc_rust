extern crate itertools;

use std::collections::HashMap;
use std::collections::VecDeque;

use itertools::Itertools;

use crate::graph::operators::color;
use crate::graph::make;
use crate::graph::types::BobbinsRef;
use crate::graph::types::LoomRef;
use crate::graph::types::SubtoursRef;
use crate::graph::types::{
    AdjacencyRef, Count, Idx, Spool,
    Yarn, VertIdxRef, VertRef, TourRef, WeightsRef, TourSliceRef,
};

pub fn from_nodes_to_yarn_ref(path: TourRef) -> Yarn {
    // convert_from_nodes
    Yarn::from(
        path.iter()
            .map(|&vert| [vert.0, vert.1])
            .collect::<Vec<[i32; 2]>>(),
    )
}

pub fn yarn(z_adj: &AdjacencyRef, vert_idx: &VertIdxRef) -> Spool {
    let weights: HashMap<VertRef, i32> = make::weights_ref::make_weights_ref(vert_idx);
    let path: TourRef = spin(&z_adj, &weights);
    let natural: Yarn = from_nodes_to_yarn_ref(path);
    let colored: Yarn = color(&natural);
    Spool::from([(3, natural), (1, colored)])
}

pub fn spin<'a>(adj: &'a AdjacencyRef, weights: &HashMap<VertRef, i32>) -> TourRef<'a> {
    let path: &mut TourRef = &mut vec![*adj.keys().max().unwrap()];
    let order: Count = adj.len();
    for idx in 1..order {
        path.push(if idx < order - 5 {
            get_next(&path, adj, weights)
        } else {
            get_next_xyz(&path, adj, weights)
        })
    }
    path.to_vec()
}

pub fn get_axis_ref2(m_vert: &(i32, i32, i32), n_vert: &(i32, i32, i32)) -> usize {
    let m_arr: &[i32; 3] = &[m_vert.0, m_vert.1, m_vert.2];
    let n_arr: &[i32; 3] = &[n_vert.0, n_vert.1, n_vert.2];
    (0..2)
        .find(|&i| m_arr[i] != n_arr[i])
        .expect("VERTS ARE SIMILAR")
}

pub fn get_axis_ref(m_vert: &(i32, i32, i32), n_vert: &(i32, i32, i32)) -> usize {
    (0..2)
        .find(|&i| match i {
            0 => m_vert.0 != n_vert.0,
            1 => m_vert.1 != n_vert.1,
            _ => true
        })
        .expect("VERTS ARE SIMILAR")
}

pub fn get_next<'a>(path: TourSliceRef, adj: &'a AdjacencyRef, weights: &WeightsRef) -> VertRef<'a> {
    adj.get(path.last().unwrap())
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .copied()
        .max_by_key(|&n| *weights.get(&n).unwrap())
        .unwrap()
}

pub fn get_next_xyz<'a>(path: TourSliceRef, adj: &'a AdjacencyRef, weights: &WeightsRef) -> VertRef<'a>  {
    let curr: &VertRef = path.last().unwrap();
    adj.get(curr)
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .map(|&n| (n, get_axis_ref(curr, n)))
        .filter(|(_, next_axis)| {
            *next_axis != get_axis_ref(&path[path.len() - 2], &curr)
        })
        .max_by_key(|&(n, _)| weights[&n])
        .unwrap()
        .0
}

pub fn wind<'a, 'b, 'c>(loom: Vec<VecDeque<&'c (i32, i32, i32)>>, vert_idx: &HashMap<(i32, i32, i32), &'c (i32, i32, i32)>) -> (BobbinsRef<'a>, LoomRef<'c>)
where
    'b: 'a,
    'c: 'a,
{

    let mut new_loom = loom.clone();
    
    (
        new_loom
            .iter_mut()
            .map(|thread| {
                let lv = thread.front().unwrap();
                let rv = thread.back().unwrap();
                let left: &'c (i32, i32, i32) = *vert_idx.get(&(lv.0, lv.1, lv.2 + 2)).unwrap();
                let right: &'c (i32, i32, i32) = *vert_idx.get(&(rv.0, rv.1, rv.2 + 2)).unwrap();
                thread.push_front(left);
                thread.push_back(right);
                vec![left, right]
            })
            .flatten()
            .collect(), 
        loom
    )
    
}


pub fn cut<'a>(tour: TourRef<'a>, subset: BobbinsRef) -> SubtoursRef<'a> {
    let mut subtours: SubtoursRef = Vec::new();
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
