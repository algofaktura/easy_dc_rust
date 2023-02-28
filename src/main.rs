pub mod data;
pub mod enums;
pub mod graphs;
pub mod info;
pub mod macros;
pub mod operators;
pub mod types;
pub mod utils;

pub mod structs;

use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use ndarray::{Array2, Axis, Slice};
use structs::vector2d::color;

use crate::data::vertices::verts32::VERTS_32;
use crate::graphs::stratify::shrink_adjacency;
use crate::graphs::make_weights::make_weights;
use crate::graphs::graph32::GRAPH;
use crate::graphs::translate::{graph_to_map, translate_verts_3d, make_vi_mapping};
use crate::info::certify::SequenceID;
use crate::info::certify::{id_seq, SequenceID::HamCycle};
use crate::operators::cut::cut;
use crate::operators::spin::spin;
use crate::operators::wind::wind;
use crate::structs::vector2d::translate_from_nodes;
use crate::structs::vector3d::Vector3D;
use crate::utils::time::elapsed_ms;

const REPEATS: u32 = 1_000;

fn main() {
    let adj: HashMap<u32, HashSet<u32>> = graph_to_map(&GRAPH);
    let v3verts: Vec<Vector3D> = translate_verts_3d(&VERTS_32);
    let vert_idx: HashMap<&Vector3D, u32> = make_vi_mapping(&v3verts);

    let (z_adj, z_length) = shrink_adjacency(&v3verts, &adj);
    let start: Instant = Instant::now();
    for _i in 0..=REPEATS { 
        weave(&z_adj, &z_length, &vert_idx, &v3verts);
    }
    elapsed_ms(start, Instant:: now(), REPEATS, "spin_nodes");

    let woven: Vec<VecDeque<u32>> = weave(&z_adj, &z_length, &vert_idx, &v3verts);
    println!("WOVEN {:?} | LEN {:?}", woven, woven[0].len());

    let seq: [u32; 32] = woven[0].iter().map(|&x| x as u32).collect::<Vec<u32>>().try_into().unwrap();
    let id: SequenceID = id_seq(&seq, &adj);
    assert_eq!(HamCycle, id);
    println!("{:?}", id);
}

fn weave(z_adj: &HashMap<u32, HashSet<u32>>, z_length: &Vec<(i32, usize)>, vert_idx: &HashMap<&Vector3D, u32>, v3verts: &Vec<Vector3D>) -> Vec<VecDeque<u32>> {
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
            .map(|row| {
                *vert_idx.get(&Vector3D::new(row[0], row[1], *zlevel as i32)).unwrap()
            })
            .collect();
        if bobbins.is_empty() {
            warps = vec![node_yarn];
        } else {
            warps = cut(node_yarn, &bobbins);
        }
        for thread in &mut loom {
            for (idx, warp) in warps.iter().enumerate() {
                if !woven.contains(&idx) {
                    for end in vec![0 as usize, thread.len() - 1] {
                        if thread[end] == warp[0 as usize] {
                            woven.extend([idx]);
                            for node in &warp[1..] {
                                if end == 0 as usize {
                                    thread.push_front(*node)
                                } else {
                                    thread.push_back(*node)
                                }
                            }
                        }
                    }
                }
            }
        }
        for (_, seq) in warps.iter().enumerate().filter(|(idx, _)| !woven.contains(idx)) {
            loom.extend(vec![VecDeque::from(seq.iter().cloned().collect::<VecDeque<u32>>())]);
        }
        let v3verts: &Vec<Vector3D> = &translate_verts_3d(&VERTS_32);
        if *zlevel != -1 {
            bobbins = wind(&mut loom, v3verts, &vert_idx);
        }
    }
    for w in &mut loom {
        let nodes: Vec<u32> = w.iter().map(|&node| {
            vert_idx.get(&v3verts[node as usize].mirror_z()).unwrap().clone()
        }).collect();
        w.extend(nodes.into_iter().rev());
    }
    loom.sort_by_key(|w| w.len());
    loom
}

fn spool_yarn(z_adj: &HashMap<u32, HashSet<u32>>) -> HashMap<u32, Array2<i32>>{
    let verts = &VERTS_32.iter().clone().map(|&(x, y, _)| (x, y)).collect::<Vec<_>>();
    let weights: HashMap<u32, i32> = make_weights(&z_adj, &VERTS_32);
    let natural: Array2<i32> = translate_from_nodes(spin(&z_adj, &weights), &verts);
    let colored: Array2<i32> = color(&natural);
    HashMap::from([(3, natural), (1, colored)])
}