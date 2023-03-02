use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z}
    }
    
    pub fn get_upper_node(&self, vert_idx: &HashMap<&Vector3D, u32>) -> u32 {
        vert_idx.get(&Vector3D { x: self.x, y: self.y, z: self.z + 2 }).unwrap().clone()
    }

    pub fn mirror_z(&self, vert_idx: &HashMap<&Vector3D, u32>) -> u32 {
        vert_idx.get(&Vector3D { x: self.x, y: self.y, z: -self.z }).unwrap().clone()
    }

    pub fn to_node(x: i32, y:i32, z: i32, vert_idx: &HashMap<&Vector3D, u32>) -> u32 {
        vert_idx.get(&Vector3D { x, y, z }).unwrap().clone()
    }
}
