use std::collections::HashMap;

use crate::graph::types::Verts;

use super::super::types::{Vectors3d, VertIdx, Verti16};
pub type Vert = (i32, i32, i32);
pub type VertRef<'a> = &'a (i32, i32, i32);
pub type VertIdxRef<'a> = HashMap<&'a Vert, VertRef<'a>>;

pub fn make_vi_mapping<'a>(verts: &'a Vec<(i32, i32, i32)>) -> HashMap<(i32, i32, i32), &'a (i32, i32, i32)> {
    verts
        .iter()
        .map(| vert| (*vert, vert))
        .collect::<HashMap<_, _>>()
}

pub fn make_vi_mapping2(verts: &Vectors3d) -> VertIdx {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (vert, idx as u32))
        .collect::<VertIdx>()
}

pub fn make_vi_mapping3(verts: &Verts) -> HashMap<Vert, u32> {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as u32))
        .collect::<HashMap<Vert, u32>>()
}

pub fn make_vi_mapping_i16(verts: &Vec<Verti16>) -> HashMap<Verti16, u32> {
    verts
        .iter()
        .enumerate()
        .map(|(idx, vert)| (*vert, idx as u32))
        .collect::<HashMap<Verti16, u32>>()
}