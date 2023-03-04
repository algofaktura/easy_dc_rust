use crate::types::types::{Node, Path, Point, PathSlice, Vectors2d, Vectors3d, VertIdx, VertsC2, Vert2dd, Yarn};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3D {
    pub x: Point,
    pub y: Point,
    pub z: Point,
}

impl Vector3D {
    pub fn new(x: Point, y: Point, z: Point) -> Self {
        Self { x, y, z }
    }

    pub fn get_upper_node(&self, vert_idx: &VertIdx) -> Node {
        vert_idx
            .get(&Vector3D {
                x: self.x,
                y: self.y,
                z: self.z + 2,
            })
            .unwrap()
            .clone()
    }

    pub fn mirror_z(&self, vert_idx: &VertIdx) -> Node {
        vert_idx
            .get(&Vector3D {
                x: self.x,
                y: self.y,
                z: -self.z,
            })
            .unwrap()
            .clone()
    }

    pub fn to_node(x: Point, y: Point, z: Point, vert_idx: &VertIdx) -> Node {
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

pub fn convert_to_2d(vec3ds: &Vectors3d) -> Vectors2d {
    vec3ds.iter().map(|v| v.to_2d()).collect()
}

pub fn convert_from_3d(vec3ds: &Vectors3d) -> Vectors2d {
    vec3ds.iter().map(|v| Vector2D::from_3d(*v)).collect()
}

pub fn convert_from_nodes(path: Path, verts: &Vert2dd) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i32; 2]>>(),
    )
}

pub fn convert_from_nodes_slice(path: PathSlice, verts: &VertsC2) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[i32; 2]>>(),
    )
}

pub fn convert_from_nodes_general<T>(path: &[T], verts: &VertsC2) -> Yarn
where
    T: TryInto<usize> + Copy,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
{
    Yarn::from(
        path.iter()
            .map(|&n| {
                let vector = verts[n.try_into().unwrap()];
                [vector.0, vector.1]
            })
            .collect::<Vec<[i32; 2]>>(),
    )
}