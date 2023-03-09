use std::collections::HashMap;

use crate::graph::types::{
    Node, Pointi16, Tour, TourSlice, VertsC2i16, Yarni16, Vert2ddi16,
};
pub type Vert2di16 = (Pointi16, Pointi16);
pub type Vert3di16 = (Pointi16, Pointi16, Pointi16);
pub type Vectors2di16 = Vec<Vector2Di16>;
pub type Vectors3di16 = Vec<Vector3Di16>;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3Di16 {
    pub x: Pointi16,
    pub y: Pointi16,
    pub z: Pointi16,
}

impl Vector3Di16 {
    pub fn new(x: Pointi16, y: Pointi16, z: Pointi16) -> Self {
        Self { x, y, z }
    }

    pub fn get_upper_node(&self, vert_idx: &HashMap<(Pointi16, Pointi16, Pointi16), u32>) -> Node {
        *vert_idx
            .get(&(self.x, self.y, self.z + 2))
            .unwrap()
    }

    pub fn mirror_z(&self, vert_idx: &HashMap<(Pointi16, Pointi16, Pointi16), u32>) -> Node {
        *vert_idx
            .get(&(self.x, self.y, -self.z))
            .unwrap()
    }

    pub fn to_node(x: Pointi16, y: Pointi16, z: Pointi16, vert_idx: &HashMap<(Pointi16, Pointi16, Pointi16), u32>) -> Node {
        *vert_idx.get(&(x, y, z)).unwrap()
    }

    pub fn to_2d(&self) -> Vector2Di16 {
        Vector2Di16 {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2Di16 {
    pub x: Pointi16,
    pub y: Pointi16,
}

impl Vector2Di16 {
    pub fn new(x: Pointi16, y: Pointi16) -> Self {
        Self { x, y }
    }

    pub fn from_3d(vector: Vector3Di16) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
        }
    }
}

pub fn convert_to_2d(vec3ds: &Vectors3di16) -> Vectors2di16 {
    vec3ds.iter().map(|v| v.to_2d()).collect()
}

pub fn convert_from_3d(vec3ds: &Vectors3di16) -> Vectors2di16 {
    vec3ds.iter().map(|v| Vector2Di16::from_3d(*v)).collect()
}

pub fn convert_from_nodes(path: Tour, verts: &Vert2ddi16) -> Yarni16 {
    Yarni16::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i16; 2]>>(),
    )
}

pub fn convert_from_nodes_slice(path: TourSlice, verts: &VertsC2i16) -> Yarni16 {
    Yarni16::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i16; 2]>>(),
    )
}

pub fn convert_from_nodes_general<T>(path: &[T], verts: &VertsC2i16) -> Yarni16
where
    T: TryInto<usize> + Copy,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Yarni16::from(
        path.iter()
            .map(|&n| {
                let vector = verts[n.try_into().unwrap()];
                [vector.0, vector.1]
            })
            .collect::<Vec<[i16; 2]>>(),
    )
}
