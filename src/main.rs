pub mod data;
pub mod enums;
pub mod graphs;
pub mod info;
pub mod operators;
pub mod structs;
pub mod types;
pub mod utils;

use ndarray::{Array2, Axis, Slice};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use crate::graphs::data::g_280::{VERTS, ADJ, EDGES};
use crate::graphs::make::{make_weights, make_vi_mapping, make_edges_adj, graph_to_map, shrink_adjacency, translate_verts_3d};
use crate::graphs::info::certify::id_seq;
use crate::graphs::info::certify::{SequenceID, SequenceID::HamCycle};
use crate::operators::operators::{cut, spin, wind};
use crate::operators::operators::color;
use crate::structs::vector2d::convert_from_nodes;
use crate::structs::vector3d::Vector3D;
use crate::structs::cycle::Cycle;
use crate::utils::time::elapsed_ms;

const REPEATS: u32 = 10_000;

fn main() {
    let adj: HashMap<u32, HashSet<u32>> = graph_to_map(&ADJ);
    let v3verts: Vec<Vector3D> = translate_verts_3d(&VERTS);
    let vert_idx: HashMap<&Vector3D, u32> = make_vi_mapping(&v3verts);
    let edge_adj: HashMap<(u32, u32), HashSet<(u32, u32)>> = make_edges_adj(&adj, &EDGES.iter().cloned().collect::<HashSet<(u32, u32)>>());
    let mut solution: Vec<u32> = Vec::new();

    let start: Instant = Instant::now();
    for _i in 0..=REPEATS { 
        solution = weave(&v3verts, &adj, &vert_idx, &edge_adj) 
    }
    elapsed_ms(start, Instant:: now(), REPEATS, "spin_nodes");

    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(HamCycle, id);
    println!("{:?}", id);
    println!("⭕️ ORDER: {:?} | ID: {:?} | {:?}", ADJ.len(), id, solution);
}

fn weave(v3verts: &Vec<Vector3D>, adj: &HashMap<u32, HashSet<u32>>, vert_idx: &HashMap<&Vector3D, u32>, edge_adj: &HashMap<(u32, u32), HashSet<(u32, u32)>> ) -> Vec<u32> {
    let mut warp_wefts = warp_loom(v3verts, &adj, vert_idx);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp = Cycle::new(warp, &adj, &edge_adj);
    let loom: HashMap<usize, &mut Cycle> = wefts.iter().enumerate().map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, &edge_adj))).collect();
    let mut processed: HashSet<usize> = HashSet::new();
    if loom.keys().len() > 0 {
        'weaving: loop {
            for idx in loom.keys() {
                if processed.len() == loom.keys().len() { break 'weaving };
                if processed.len() - 1 == loom.keys().len() { warp.set_last() };
                if processed.contains(idx) { continue };
                let mut bridge = warp.edges().intersection(&loom[idx].eadjs()).into_iter().cloned().collect::<HashSet<(u32, u32)>>();
                if !bridge.is_empty() {
                    let warp_e = bridge.drain().next().unwrap();
                    let mut other = loom[&*idx].clone();
                    let mut weft_es = edge_adj.get(&warp_e).unwrap().intersection(&other.edges()).into_iter().cloned().collect::<HashSet<(u32, u32)>>();
                    if !weft_es.is_empty() {
                        let weft_e = weft_es.drain().next().unwrap();
                        warp.join(warp_e, weft_e, &mut other);
                        processed.extend([idx]);
                    }
                }
            }
        }
    }
    warp.retrieve()
}
    
fn warp_loom(v3verts: &Vec<Vector3D>, adj: &HashMap<u32, HashSet<u32>>, vert_idx: &HashMap<&Vector3D, u32>) -> Vec<VecDeque<u32>> {
    let (z_adj, z_length) = shrink_adjacency(&v3verts, &adj);
    let spool: HashMap<u32, Array2<i32>> = spool_yarn(&z_adj);
    let mut bobbins: Vec<u32> = Vec::new();
    let mut warps: Vec<Vec<u32>>;
    let mut loom: Vec<VecDeque<u32>> = Vec::new();
    for (zlevel, order) in z_length {
        let mut yarn: Array2<i32> = spool[&(zlevel % 4 + 4).try_into().unwrap()].clone();
        yarn.slice_axis_inplace(Axis(0), Slice::new((yarn.len_of(Axis(0)) - order).try_into().unwrap(), None, 1));
        let node_yarn: Vec<u32> = yarn.outer_iter().map(|row| Vector3D::to_node(row[0], row[1], zlevel, &vert_idx)).collect();
        if bobbins.is_empty() { warps = vec![node_yarn] } else { warps = cut(node_yarn, &bobbins) }
        let mut woven: HashSet<usize> = HashSet::new();
        for thread in &mut loom {
            for (idx, warp) in warps.iter().enumerate() {
                if !woven.contains(&idx) {
                    for end in vec![0 as usize, thread.len() - 1] {
                        if thread[end] == warp[0 as usize] {
                            woven.extend([idx]);
                            for node in &warp[1..] {
                                if end == 0 as usize { thread.push_front(*node) } else { thread.push_back(*node) }
                            }
                        }
                    }
                }
            }
        }
        for (_, seq) in warps.iter().enumerate().filter(|(idx, _)| !woven.contains(idx)) {
            loom.extend(vec![VecDeque::from(seq.iter().cloned().collect::<VecDeque<u32>>())]);
        }
        let v3verts: &Vec<Vector3D> = &translate_verts_3d(&VERTS);
        if zlevel != -1 { bobbins = wind(&mut loom, v3verts, &vert_idx) }
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