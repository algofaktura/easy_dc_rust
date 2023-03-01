pub mod data;
pub mod enums;
pub mod graphs;
pub mod info;
pub mod macros;
pub mod operators;
pub mod types;
pub mod utils;

pub mod structs;

use ndarray::{Array2, Axis, Slice};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::data::g_280::{VERTS, ADJ};
use crate::graphs::reduce::shrink_adjacency;
use crate::graphs::make::{make_weights, make_vi_mapping};
use crate::graphs::translate::{graph_to_map, translate_verts_3d};
use crate::operators::operators::{cut, spin, wind};
use crate::operators::operators::color;
use crate::structs::vector2d::convert_from_nodes;
use crate::structs::vector3d::Vector3D;


fn main() {
    println!("⭕️ ORDER: {:?}", ADJ.len());
    let adj: HashMap<u32, HashSet<u32>> = graph_to_map(&ADJ);
    let v3verts: Vec<Vector3D> = translate_verts_3d(&VERTS);
    let vert_idx: HashMap<&Vector3D, u32> = make_vi_mapping(&v3verts);
    let (z_adj, z_length) = shrink_adjacency(&v3verts, &adj);
    let woven: Vec<VecDeque<u32>> = warp_loom(&z_adj, &z_length, &vert_idx, &v3verts);
    println!("WOVEN {:?} | LEN {:?}", woven, woven.len());

}

fn warp_loom(z_adj: &HashMap<u32, HashSet<u32>>, z_length: &Vec<(i32, usize)>, vert_idx: &HashMap<&Vector3D, u32>, v3verts: &Vec<Vector3D>) -> Vec<VecDeque<u32>> {
    let spool: HashMap<u32, Array2<i32>> = spool_yarn(z_adj);
    let mut bobbins: Vec<u32> = Vec::new();
    let mut loom: Vec<VecDeque<u32>> = Vec::new();
    let mut warps: Vec<Vec<u32>>;
    for (zlevel, order) in z_length {
        let mut woven: HashSet<usize> = HashSet::new();
        let mut yarn1: Array2<i32> = spool[&(zlevel % 4 + 4).try_into().unwrap()].clone();
        yarn1.slice_axis_inplace(Axis(0), Slice::new((yarn1.len_of(Axis(0)) - order).try_into().unwrap(), None, 1));
        let node_yarn: Vec<u32> = yarn1
            .outer_iter()
            .map(|row| *vert_idx.get(&Vector3D::new(row[0], row[1], *zlevel as i32)).unwrap())
            .collect();
        if bobbins.is_empty() { warps = vec![node_yarn] } else { warps = cut(node_yarn, &bobbins) }
        for thread in &mut loom {
            for (idx, warp) in warps.iter().enumerate() {
                if !woven.contains(&idx) {
                    for end in vec![0 as usize, thread.len() - 1] {
                        if thread[end] == warp[0 as usize] {
                            woven.extend([idx]);
                            for node in &warp[1..] {
                                if end == 0 as usize { thread.push_front(*node) } else { thread.push_back(*node) }
                            }
        }}}}}
        for (_, seq) in warps.iter().enumerate().filter(|(idx, _)| !woven.contains(idx)) {
            loom.extend(vec![VecDeque::from(seq.iter().cloned().collect::<VecDeque<u32>>())]);
        }
        let v3verts: &Vec<Vector3D> = &translate_verts_3d(&VERTS);
        if *zlevel != -1 { bobbins = wind(&mut loom, v3verts, &vert_idx) }
    }
    for w in &mut loom {
        let nodes: Vec<u32> = w.iter().map(|&node| v3verts[node as usize].mirror_z(&vert_idx)).collect();
        w.extend(nodes.into_iter().rev());
    }
    loom.sort_by_key(|w| w.len());
    loom
}

fn spool_yarn(z_adj: &HashMap<u32, HashSet<u32>>) -> HashMap<u32, Array2<i32>>{
    let verts = &VERTS.iter().clone().map(|&(x, y, _)| (x, y)).collect::<Vec<_>>();
    let weights: HashMap<u32, i32> = make_weights(&z_adj, &VERTS);
    let natural: Array2<i32> = convert_from_nodes(spin(&z_adj, &weights), &verts);
    let colored: Array2<i32> = color(&natural);
    HashMap::from([(3, natural), (1, colored)])
}

// make ea: just translate it or make a function that makes it.. so first just translate it.
// fn weave(warp: VecDeque<u32>, wefts: Vec<VecDeque<u32>>) -> Cycle {
pub fn weave(warp: VecDeque<u32>, adj: &HashMap<u32, HashSet<u32>>, edge_adj: HashMap<(u32, u32), HashSet<(u32, u32)>>) {
    let data = warp.iter().cloned().collect::<Vec<_>>();
    // Cycle::new(data, adj, edge_adj)
    println!("{:?} {:?} {:?}", data, edge_adj, adj);
    }