use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug, Eq, Hash)]
pub enum Axis {
    S,
    X,
    Y,
    Z,
}

impl Into<usize> for Axis {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<Axis> for usize {
    fn into(self) -> Axis {
        match self {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            3 => Axis::S,
            _ => panic!("Invalid axis value: {}", self),
        }
    }
}

pub fn get_edge_axis(m_vert: &[i32; 3], n_vert: &[i32; 3]) -> Axis {
    match (0..2).find(|&i| m_vert[i] != n_vert[i]) {
        Some(i) => i.into(),
        None => Axis::S,
    }
}

pub fn edge_axis(m_vert: &[i32; 3], n_vert: &[i32; 3]) -> usize {
    (0..2).find(|&i| m_vert[i] != n_vert[i]).unwrap()
}

pub fn spin(
    adj_map: &HashMap<u32, HashSet<u32>>,
    weights: &HashMap<u32, i32>,
    verts: &[[i32; 3]; 448],
) -> Vec<u32> {
    let path = &mut vec![*adj_map.keys().max().unwrap() as u32];
    let order = adj_map.len();
    let limit: usize = order - 5;
    for idx in 1..order {
        path.push(if idx < limit {
            get_next(path.to_vec(), adj_map, weights)
        } else {
            get_next_xyz(path.to_vec(), adj_map, weights, verts)
        })
    }
    path.to_vec()
}

pub fn get_next(
    path: Vec<u32>,
    adj_map: &HashMap<u32, HashSet<u32>>,
    weights: &HashMap<u32, i32>,
) -> u32 {
    adj_map
        .get(path.last().unwrap())
        .unwrap()
        .difference(&path.iter().cloned().collect::<HashSet<u32>>())
        .cloned()
        .max_by_key(|&n| *weights.get(&n).unwrap())
        .unwrap()
}

pub fn get_next_xyz(
    path: Vec<u32>,
    adj_map: &HashMap<u32, HashSet<u32>>,
    weights: &HashMap<u32, i32>,
    verts: &[[i32; 3]; 448],
) -> u32 {
    let curr = path.last().unwrap();
    let curr_vert = &verts[*curr as usize];
    let prev_vert = &verts[path[path.len() - 2] as usize];
    adj_map
        .get(curr)
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .map(|&n| (n, edge_axis(curr_vert, &verts[n as usize])))
        .filter(|(_, next_axis)| *next_axis != edge_axis(prev_vert, curr_vert))
        .max_by_key(|(n, _)| weights[n])
        .unwrap()
        .0
}
