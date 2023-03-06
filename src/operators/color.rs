use ndarray::arr2;

use crate::types::types::Yarn;

pub fn color(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn reflect(a: &Yarn) -> Yarn {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Yarn) -> Yarn {
    a + arr2(&[[0, 2]])
}
