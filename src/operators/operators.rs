use ndarray::arr2;

use crate::types::types::{Idx, V3d, Yarn};

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

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn reflect(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Yarn) -> Yarn {
    a + arr2(&[[0, 2]])
}
