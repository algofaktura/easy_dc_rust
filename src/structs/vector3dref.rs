use std::collections::HashMap;

type VertIdx<'a> = HashMap<(i32, i32, i32), &'a (i32, i32, i32)>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3dRef<'a> {
    pub data: &'a (i32, i32, i32),
}

impl<'a> Vector3dRef<'a> {
    pub fn new(data: &'a (i32, i32, i32)) -> Self {
        Self { data }
    }

    pub fn get_upper_node(&'a self, vert_idx: &'a VertIdx) -> &(i32, i32, i32) {
        *vert_idx
            .get(&(self.data.0, self.data.1, self.data.2 + 2))
            .unwrap()
    }

    pub fn mirror_z(&'a self, vert_idx: &'a VertIdx) -> &(i32, i32, i32) {
        *vert_idx
            .get(&(self.data.0, self.data.1, -self.data.2))
            .unwrap()
    }

    pub fn to_node(&self, vert_idx: &'a VertIdx) -> &(i32, i32, i32) {
        *vert_idx.get(&*self.data).unwrap()
    }

    pub fn to_2d(&self) -> Vector2D {
        Vector2D {
            x: self.data.0,
            y: self.data.1,
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

    pub fn from_3d(vector: Vector3dRef) -> Self {
        Self {
            x: vector.data.0,
            y: vector.data.1,
        }
    }
}
