use ndarray::{arr2, Array2};
use std::time::{Duration, Instant};

use super::types::{Idx, Point, V3d, Vert, Yarn, Adjacency};

pub fn get_axis(m_vert: &V3d, n_vert: &V3d) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("VERTS ARE SIMILAR")
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

pub fn absumv_v3d(vert: V3d) -> Point {
    vert.iter()
        .map(|&n| ((n >> 31) ^ n).wrapping_sub(n >> 31))
        .sum()
}

pub fn edist((x, y, z): Vert) -> Point {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt().round() as i32
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

pub fn get_order_from_n(n: u32) -> u32{
    ((4.0 / 3.0) * (n as f64 + 2.0) * (n as f64 + 1.0) * n as f64).round() as u32
}

pub fn elapsed_ms(start: Instant, end: Instant, _repeats: u32, _name: &str) -> f64 {
    let dur: Duration = end - start;
    // println!("x{repeats}: {name}() took {} secs", dur.as_secs_f64());
    dur.as_secs_f64()
}

pub fn sum_neighbors(adj: &Adjacency) -> usize {
    adj 
        .values()
        .map(
            |value|
            value.len()
        )
        .sum()
}

pub fn uon(start: usize, end: usize, max_n: usize) -> impl Iterator<Item = usize> {
    (0..max_n + 2)
        .map(move |i| {
            let _uon = (0..max_n * 2 + 2)
                .step_by(2)
                .take(i)
                .map(|n| n * (n + 2))
                .sum();
            if _uon >= start && _uon <= end {
                Some(_uon)
            } else {
                None
            }
        })
        .flatten()
}
