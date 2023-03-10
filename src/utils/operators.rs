use ndarray::{arr2, Array2};
use ndarray::{Axis, Slice};

use std::time::{Duration, Instant};

use super::super::graph::types::{
    Adjacency, Count, Idx, Node, Point, Tour, TourSlice, V3d, VIMap, Varr, Vert, Weights, Yarn,
};

pub fn get_axis(m_vert: &V3d, n_vert: &V3d) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("VERTS ARE SIMILAR")
}

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn reflect(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift_2y(a: Yarn) -> Yarn {
    a + arr2(&[[0, 2]])
}

pub fn shift_xyz(vert: Array2<Point>) -> Array2<Point> {
    vert + arr2(&[
        [2, 0, 0],
        [-2, 0, 0],
        [0, 2, 0],
        [0, -2, 0],
        [0, 0, 2],
        [0, 0, -2],
    ])
}

pub fn absumv((x, y, z): Vert) -> Point {
    [x, y, z]
        .iter()
        .map(|&n| ((n >> 31) ^ n).wrapping_sub(n >> 31))
        .sum()
}

pub fn get_upper_nodes((x, y, z): Vert, (x1, y1, z1): Vert, vert_idx: &VIMap) -> (u32, u32) {
    (vert_idx[&(x, y, z + 2)], vert_idx[&(x1, y1, z1 + 2)])
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

pub fn get_next_xyz(path: TourSlice, adj: &Adjacency, weights: &Weights, verts: &Varr) -> Node {
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

pub fn get_node_yarn(mut yarn: Yarn, zlevel: Point, order: Count, vert_idx: &VIMap) -> Tour {
    yarn.slice_axis_inplace(
        Axis(0),
        Slice::new((yarn.len_of(Axis(0)) - order).try_into().unwrap(), None, 1),
    );
    yarn.outer_iter()
        .map(|row| vert_idx[&(row[0], row[1], zlevel)])
        .collect()
}

pub fn edist((x, y, z): Vert) -> Point {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt().round() as i32
}

pub fn edist_f32((x, y, z): Vert) -> f32 {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt()
}

pub fn get_max_xyz(order: i32) -> Point {
    (0..order)
        .map(|n| {
            (
                n,
                ((4.0 / 3.0) * (n as f64 + 2.0) * (n as f64 + 1.0) * n as f64).round() as u32,
            )
        })
        .filter(|(_, sum)| *sum == order as u32)
        .map(|(n, _)| n)
        .next()
        .unwrap()
        * 2
        - 1
}

pub fn elapsed_ms(start: Instant, end: Instant, repeats: u32, name: &str) -> f64 {
    let dur: Duration = end - start;
    println!("x{repeats}: {name}() took {} secs", dur.as_secs_f64());
    dur.as_secs_f64()
}
