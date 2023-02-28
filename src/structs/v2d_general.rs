pub fn translate_from_nodes<T>(path: Vec<u32>, verts: &[(T, T, T)]) -> Vec<Vector2D<T>>
where
    T: Copy,
{
    let vertices = verts
        .iter()
        .map(|v| Vector3D::<T> { x: v.0, y: v.1, z: v.2 })
        .collect::<Vec<_>>();
    let nodes = path.iter().map(|&n| vertices[n as usize]).collect::<Vec<_>>();
    nodes.into_iter().map(Vector2D::<T>::from_3d).collect()
}

pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_3d(vector: Vector3D<T>) -> Self {
        Self { x: vector.x, y: vector.y }
    }
}

pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}