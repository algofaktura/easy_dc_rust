use std::collections::HashMap;

use ndarray::Array2;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn get_upper_node(&self, vert_idx: &HashMap<&Vector3D, u32>) -> u32 {
        vert_idx
            .get(&Vector3D {
                x: self.x,
                y: self.y,
                z: self.z + 2,
            })
            .unwrap()
            .clone()
    }

    pub fn mirror_z(&self, vert_idx: &HashMap<&Vector3D, u32>) -> u32 {
        vert_idx
            .get(&Vector3D {
                x: self.x,
                y: self.y,
                z: -self.z,
            })
            .unwrap()
            .clone()
    }

    pub fn to_node(x: i32, y: i32, z: i32, vert_idx: &HashMap<&Vector3D, u32>) -> u32 {
        vert_idx.get(&Vector3D { x, y, z }).unwrap().clone()
    }

    pub fn to_2d(&self) -> Vector2D {
        Vector2D {
            x: self.x,
            y: self.y,
        }
    }
}

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
            y: vector.y,
        }
    }
}

pub fn convert_to_2d(vec3ds: &Vec<Vector3D>) -> Vec<Vector2D> {
    vec3ds.iter().map(|v| v.to_2d()).collect()
}

pub fn convert_from_3d(vec3ds: &Vec<Vector3D>) -> Vec<Vector2D> {
    vec3ds.iter().map(|v| Vector2D::from_3d(*v)).collect()
}

pub fn convert_from_nodes(path: Vec<u32>, verts: &Vec<(i32, i32)>) -> Array2<i32> {
    Array2::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i32; 2]>>(),
    )
}

pub fn convert_from_nodes_slice(path: &[u32], verts: &[(i32, i32)]) -> Array2<i32> {
    Array2::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i32; 2]>>(),
    )
}

pub fn convert_from_nodes_general<T>(path: &[T], verts: &[(i32, i32)]) -> Array2<i32>
where
    T: TryInto<usize> + Copy,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Array2::from(
        path.iter()
            .map(|&n| {
                let vector = verts[n.try_into().unwrap()];
                [vector.0, vector.1]
            })
            .collect::<Vec<[i32; 2]>>(),
    )
}
