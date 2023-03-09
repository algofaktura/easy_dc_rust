use ndarray::{arr2, Array2};

use super::{super::graph::types::{Idx, V3d, Yarn}, types::{Yarni16, V3di16}};

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
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("VERTS ARE SIMILAR")
}

pub fn get_axis_i16(m_vert: &V3di16, n_vert: &V3di16) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("VERTS ARE SIMILAR")
}

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn color_i16(a: &Yarni16) -> Yarni16 {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn reflect(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Yarn) -> Yarn {
    a + arr2(&[[0, 2]])
}

pub fn shift_xyz(vert: Array2<i32>) -> Array2<i32> {
    vert + arr2(&[
        [2, 0, 0],
        [-2, 0, 0],
        [0, 2, 0],
        [0, -2, 0],
        [0, 0, 2],
        [0, 0, -2],
    ])
}

pub fn shift_xyz_i16(vert: Array2<i16>) -> Array2<i16> {
    vert + arr2(&[
        [2, 0, 0],
        [-2, 0, 0],
        [0, 2, 0],
        [0, -2, 0],
        [0, 0, 2],
        [0, 0, -2],
    ])
}

pub fn absumv((x, y, z): (i32, i32, i32)) -> i32 {
    x.abs() + y.abs() + z.abs()
}
