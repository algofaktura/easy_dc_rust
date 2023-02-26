use ndarray::{arr2, Array2};

use crate::structs::vector3d::Vector3D;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn from_3d(vector: Vector3D) -> Self {
        Self { 
            x: vector.x, 
            y: vector.y 
        }
    }
}

pub fn convert_to_2d(vec3ds: &Vec<Vector3D>) -> Vec<Vector2D> {
    vec3ds.iter().map(|v| v.to_2d()).collect()
}

pub fn convert_from_3d(vec3ds: &Vec<Vector3D>) -> Vec<Vector2D> {
    vec3ds.iter().map(|v| Vector2D::from_3d(*v)).collect()
}

pub fn reflect(a: &Array2<i32>) -> Array2<i32> {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: &Array2<i32>) -> Array2<i32> {
    a + arr2(&[[0, 2]])
}
