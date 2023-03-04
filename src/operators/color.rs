use ndarray::{arr2, Array2};

pub fn color(a: &Array2<i32>) -> Array2<i32> {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}

pub fn reflect(a: &Array2<i32>) -> Array2<i32> {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Array2<i32>) -> Array2<i32> {
    a + arr2(&[[0, 2]])
}
