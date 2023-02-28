use std::collections::{HashMap, HashSet};

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

// use crate::structs::vector2d::Vector2D;
// use crate::graphs::translate::translate_verts_2d;

// pub fn node_to_vectors(path: Vec<u32>, verts: &[(i32, i32, i32)]) -> Vec<Vector2D>{
//     // iterate through the path and can own it because it is not needed afterward
//     // so we need to get the path and iterate through it and map it.
//     translate_verts_2d(verts)

// }

pub fn spin_slower(adj_map: &HashMap<u32, HashSet<u32>>, start: u32, weights: &HashMap<u32, i32>) -> Vec<u32> {
    let mut path = vec![start];
    for _ in 1..adj_map.len() {
        let next = adj_map.get(path.last().unwrap())
            .unwrap()
            .iter()
            .filter(|&n| !path.contains(n))
            .max_by_key(|&n| *weights.get(&n).unwrap())
            .unwrap();
        path.push(*next);
    }
    path
}

pub fn spin_slower2(adj_map: &HashMap<u32, HashSet<u32>>, start: u32, weights: &HashMap<u32, i32>) -> Vec<u32> {
    let mut path: Vec<u32> = vec![start];
    for _i in 0..adj_map.len() - 1 {
        let next_options: Vec<&u32> = adj_map.get(path.last().unwrap()).unwrap()
            .iter()
            .filter(|&n| !path.contains(&n))
            .collect::<Vec<_>>();        
        let next: &&u32 = next_options
            .iter()
            .max_by_key(|&n| *weights.get(n).unwrap())
            .unwrap();
        path.push(**next);
    }
    path
}

pub fn spinref(adj_map: &HashMap<&u32, HashSet<&u32>>, start: u32, weights: &HashMap<&u32, i32>) -> Vec<u32> {
    let mut path: Vec<u32> = vec![start];
    for _i in 0..adj_map.len() - 1 {
        let next_options: Vec<&u32> = adj_map.get(path.last().unwrap()).unwrap()
            .iter()
            .filter(|&n| !path.contains(n))
            .copied()
            .collect::<Vec<_>>();
        let next: &&u32 = next_options
            .iter()
            .max_by_key(|&n| *weights.get(n).unwrap())
            .unwrap();
        path.push(**next);
    }
    path
}