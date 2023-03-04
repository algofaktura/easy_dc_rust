use crate::types::types::*;

pub fn wind(loom: &mut Loom, verts: &Vectors3d, vert_idx: &VertIdx) -> Bobbins {
    let mut bobbins: Vec<u32> = Vec::new();
    for thread in loom.iter_mut() {
        let left: u32 = verts[thread[0] as usize].get_upper_node(&vert_idx);
        let right: u32 = verts[thread[thread.len() - 1] as usize].get_upper_node(&vert_idx);
        thread.push_front(left);
        thread.push_back(right);
        bobbins.extend(vec![left as u32, right as u32]);
    }
    bobbins
}