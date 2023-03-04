use crate::types::types::{Adjacency, Count, Idx, Neighbors, Node, Path, PathSlice, Weights, V3d, V3Slice};

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

pub fn get_edge_axis(m_vert: &V3d, n_vert: &V3d) -> Axis {
    match (0..2).find(|&i| m_vert[i] != n_vert[i]) {
        Some(i) => i.into(),
        None => Axis::S,
    }
}

pub fn get_axis(m_vert: &V3d, n_vert: &V3d) -> Idx {
    (0..2).find(|&i| m_vert[i] != n_vert[i]).expect("VERTS ARE SIMILAR")
}

pub fn spin(
    adj: &Adjacency,
    weights: &Weights,
    verts: V3Slice,
) -> Path {
    let path: &mut Path = &mut vec![*adj.keys().max().unwrap() as Node];
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

pub fn get_next(
    path: PathSlice,
    adj: &Adjacency,
    weights: &Weights,
) -> Node {
    adj
        .get(path.last().unwrap())
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .copied()
        .max_by_key(|&n| *weights.get(&n).unwrap())
        .unwrap()
}

pub fn get_next1(
    path: PathSlice,
    adj: &Adjacency,
    weights: &Weights,
) -> Node {
    *adj
        .get(path.last().expect("Path is empty"))
        .expect("No adjacent vertices found")
        .difference(&path.iter().copied().collect::<Neighbors>())
        .max_by_key(|&n| weights.get_key_value(&n).unwrap().1)
        .expect("No unvisited adjacent vertices found")
}

pub fn get_next_xyz(
    path: PathSlice,
    adj: &Adjacency,
    weights: &Weights,
    verts: V3Slice,
) -> Node {
    let curr: &Node = path.last().unwrap();
    let curr_vert: &V3d = &verts[*curr as usize];
    let prev_vert: &V3d = &verts[path[path.len() - 2] as usize];
    adj
        .get(curr)
        .unwrap()
        .iter()
        .filter(|n| !path.contains(*n))
        .map(|&n| (n, get_axis(curr_vert, &verts[n as usize])))
        .filter(|(_, next_axis)| *next_axis != get_axis(prev_vert, curr_vert))
        .max_by_key(|&(n, _)| weights[&n])
        .unwrap()
        .0
}