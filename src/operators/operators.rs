use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;

use ndarray::{Array2, arr2};

use crate::structs::vector3d::Vector3D;

pub fn spin(adj_map: &HashMap<u32, HashSet<u32>>, weights: &HashMap<u32, i32>) -> Vec<u32> {
    let mut path: Vec<u32> = vec![*adj_map.keys().max().unwrap() as u32];
    for _ in 1..adj_map.len() {
        let next = adj_map.get(path.last().unwrap()).unwrap()
            .difference(&path.iter().cloned().collect::<HashSet<u32>>())
            .cloned()
            .max_by_key(|&n| *weights.get(&n).unwrap())
            .unwrap();
        path.push(next);
    }
    path
}

pub fn cut(tour: Vec<u32>, subset: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut subtours: Vec<Vec<u32>> = vec![];
    let mut idxs: Vec<usize> = tour
        .iter()
        .enumerate()
        .filter_map(|(i, &node)| if subset.contains(&node) { Some(i) } else { None })
        .collect::<Vec<usize>>();
    idxs.sort();
    let last_ix: usize = tour.len() - 1;
    let mut prev: i32 = -1 as i32;
    for (e, idx) in idxs.iter().enumerate() {
        if e == idxs.len() - 1 && *idx != last_ix {
            for subtour in vec![tour[(prev + 1) as usize..*idx].to_vec(), tour[*idx..].to_vec()] {
                if !subtour.is_empty() {
                    if subset.contains(&subtour[0]) {
                        subtours.push(subtour)
                    } else {
                        subtours.push(subtour.into_iter().rev().collect())
                    }
                }
            }
        } else {
            let subtour: Vec<u32> = tour[(prev + 1) as usize..=*idx].to_vec();
            if !subtour.is_empty() {
                if subset.contains(&subtour[0]) {
                    subtours.push(subtour)
                } else {
                    subtours.push(subtour.iter().rev().cloned().collect())
                }
            }
            prev = *idx as i32
        }
    }
    subtours    
}

pub fn wind(loom: &mut Vec<VecDeque<u32>>, verts: &Vec<Vector3D>, vert_idx: &HashMap<&Vector3D, u32>) -> Vec<u32> {
    let mut bobbins: Vec<u32> = Vec::new();
    for thread in loom.iter_mut() {
        let left = verts[thread[0] as usize].get_upper_node(&vert_idx);
        let right = verts[thread[thread.len()-1] as usize].get_upper_node(&vert_idx);
        thread.push_front(left);
        thread.push_back(right);
        bobbins.extend(vec![left as u32, right as u32]);
    }
    bobbins
}

pub fn flip(ix: usize, path: &mut [i32], end: bool) {
    if end {
        path[ix + 1..].reverse();
    } else {
        path[..ix].reverse();
    }
}

pub fn reflect(a: &Array2<i32>) -> Array2<i32> {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Array2<i32>) -> Array2<i32> {
    a + arr2(&[[0, 2]])
}

pub fn color(a: &Array2<i32>) -> Array2<i32> {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}