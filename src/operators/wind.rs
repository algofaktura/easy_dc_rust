use std::collections::VecDeque;
use std::collections::HashMap;

use crate::structs::vector3d::Vector3D;

pub fn wind(loom: &mut Vec<VecDeque<u32>>, verts: &Vec<Vector3D>, vert_idx: &HashMap<&Vector3D, u32>) -> Vec<u32> {
    let mut bobbins: Vec<u32> = Vec::new();
    for thread in loom.iter_mut() {
        let left = vert_idx[&verts[thread[0] as usize].add_scalar_z2()];
        let right = vert_idx[&verts[thread[thread.len()-1] as usize].add_scalar_z2()];
        thread.push_front(left);
        thread.push_back(right);
        bobbins.extend(vec![left as u32, right as u32]);
    }
    bobbins
}

