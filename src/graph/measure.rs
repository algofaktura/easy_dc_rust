use super::types::{Vert, Point};

pub fn edist((x, y, z): Vert) -> Point {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt().round() as i32
}

pub fn edist_f32((x, y, z): Vert) -> f32 {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt()
}

pub fn absumv((x, y, z): Vert) -> Point {
    [x, y, z].iter().map(|&n| ((n >> 31) ^ n).wrapping_sub(n >> 31)).sum()
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
