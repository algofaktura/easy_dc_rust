use super::super::types::{Vectors3d, VertIdx,  Verts, VIMap};

pub fn make_vi_mapping_v3d(verts: &Vectors3d) -> VertIdx {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (vert, idx as u32))
        .collect::<VertIdx>()
}

pub fn make_vi_mapping(verts: &Verts) -> VIMap {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as u32))
        .collect::<VIMap>()
}
