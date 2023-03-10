use ndarray::{arr2, Array2};

use super::super::graph::types::{Idx, Point, V3d, Vert, Yarn};

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
