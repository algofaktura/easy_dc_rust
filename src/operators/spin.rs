use std::collections::{HashMap, HashSet};

pub fn spin(adj_map: &HashMap<u32, HashSet<u32>>, start: u32, weights: &HashMap<u32, i32>) -> Vec<u32> {
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

// Uses a reference but is slower than above
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

/*
Shorter but slower? could it be bc of the creation of path_set?
Is this closure: |&n| !path.contains(n) faster bc it is merely checking either 3 or 6 neighbors?
 */ 
pub fn spin_slower(adj_map: &HashMap<u32, HashSet<u32>>, start: u32, weights: &HashMap<u32, i32>) -> Vec<u32> {
    let mut path: Vec<u32> = vec![start];
    for _i in 0..adj_map.len() - 1 {
        let path_set = &path.iter().cloned().collect();
        let next = adj_map.get(path.last().unwrap()).unwrap()
            .difference(&path_set)
            .max_by_key(|&n| *weights.get(&n).unwrap())
            .unwrap();
        path.push(*next);
    }
    path
}

