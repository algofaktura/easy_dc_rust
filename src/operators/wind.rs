use std::collections::{HashSet, VecDeque};
use std::collections::HashMap;

use crate::structs::vector3d::Vector3D;

pub fn wind(loom: &mut Vec<VecDeque<i32>>, verts: &Vec<Vector3D>, vert_idx: &HashMap<Vector3D, i32>) -> HashSet<u32> {
    let mut bobbins: HashSet<u32> = HashSet::new();
    for thread in loom.iter_mut() {
        let v1 = verts[thread[0] as usize];
        let left = vert_idx[&v1.add_scalar_z2()];
        let v2 = verts[thread[thread.len()-1] as usize];
        let right = vert_idx[&v2.add_scalar_z2()];
        thread.push_front(left);
        thread.push_back(right);
        bobbins.insert(left as u32);
        bobbins.insert(right as u32);
    }
    bobbins
}
