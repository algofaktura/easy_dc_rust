use std::collections::HashMap;

use super::super::types::{Node, VIMap, Vectors3d, VertIdx, Verts};

pub fn make_vi_mapping_v3d(verts: &Vectors3d) -> VertIdx {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (vert, idx as Node))
        .collect::<VertIdx>()
}

pub fn make_vi_mapping(verts: &Verts) -> VIMap {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as Node))
        .collect::<VIMap>()
}

pub fn make_vi_mapping_general<'a, T, U>(verts: &U) -> HashMap<T, usize>
where
    T: Copy + std::hash::Hash + std::cmp::Eq + 'a,
    U: IntoIterator<Item = &'a T> + ?Sized + 'static + Copy,
{
    verts
        .into_iter()
        .enumerate()
        .map(|(idx, vert)| (vert.clone(), idx))
        .collect()
}
