use ndarray::{arr2, Array2};
use std::convert::TryInto;
use std::fmt::Debug;

use crate::structs::vector3d::Vector3D;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

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

pub fn translate_from_nodes(path: Vec<u32>, verts: &Vec<(i32, i32)>) -> Array2<i32> {
    Array2::from(path.iter().map(|&n| [verts[n as usize].0, verts[n as usize].1]).collect::<Vec<[i32; 2]>>())
}

pub fn translate_from_nodes_slice(path: &[u32], verts: &[(i32, i32)]) -> Array2<i32> {
    Array2::from(path.iter().map(|&n| [verts[n as usize].0, verts[n as usize].1]).collect::<Vec<[i32; 2]>>())
}

pub fn translate_from_nodes_gen2<T>(path: &[T], verts: &[(i32, i32)]) -> Array2<i32>
where
    T: TryInto<usize> + Copy,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Array2::from(path.iter().map(|&n| {
        let vector = verts[n.try_into().unwrap()];
        [vector.0, vector.1]
    }).collect::<Vec<[i32; 2]>>())
}

pub fn reflect(a: &Array2<i32>) -> Array2<i32> {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]]))
}

pub fn shift(a: Array2<i32>) -> Array2<i32> {
    a + arr2(&[[0, 2]])
}

pub fn color(a: &Array2<i32>) -> Array2<i32> {
    a.clone().dot(&arr2(&[[-1, 0], [0, -1]])) + arr2(&[[0, 2]])
}