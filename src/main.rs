use ndarray::{Axis, Slice};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

pub mod graphs;
pub mod operators;
pub mod structs;
pub mod types;
pub mod utils;

use graphs::data::g_16192::{VERTS, ADJ, EDGES};
use graphs::utils::make::{make_weights, make_vi_mapping, make_edges_adj};
use graphs::utils::map::{map_graph, vectorize, convert_from_nodes};
use graphs::utils::shrink::shrink_adjacency;
use graphs::info::certify::{id_seq, SequenceID, SequenceID::HamCycle};
use operators::operators::{cut, spin, wind, color};
use structs::vector::Vector3D;
use structs::cycle::Cycle;
use utils::time::elapsed_ms;
use types::types::*;

const REPEATS: u32 = 1;

fn main() {
    let adj: Adjacency = map_graph(&ADJ);
    let v3verts: Vectors3d = vectorize(&VERTS);
    let vert_idx: VertIdx = make_vi_mapping(&v3verts);
    let edge_adj: EdgeAdjacency = make_edges_adj(&adj, &EDGES.iter().cloned().collect::<Edges>());
    let mut solution: Solution = Vec::new();

    let start: Instant = Instant::now();
    for _i in 0..=REPEATS { 
        solution = weave(&v3verts, &adj, &vert_idx, &edge_adj) 
    }
    elapsed_ms(start, Instant:: now(), REPEATS, "weave");

    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(HamCycle, id);
    println!("{:?}", id);
    println!("⭕️ ORDER: {:?} | ID: {:?} | {:?}", ADJ.len(), id, solution.len());
}

fn weave(v3verts: &Vectors3d, adj: &Adjacency, vert_idx: &VertIdx, edge_adj: &EdgeAdjacency) -> Solution {
    let mut warp_wefts: Wefts = warp_loom(v3verts, &adj, vert_idx);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut Cycle = Cycle::new(warp, &adj, &edge_adj, true);
    let loom: WarpedLoom = wefts.iter().enumerate().map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, &edge_adj, false))).collect();
    let mut processed: Processed = HashSet::new();
    if loom.keys().len() > 0 {
        'weaving: loop {
            for idx in loom.keys() {
                if processed.len() == loom.keys().len() { break 'weaving }
                if processed.contains(idx) { continue }
                if processed.len() - 1 == loom.keys().len() { warp.set_last() }
                let mut other: Cycle = loom[&*idx].clone();
                let mut bridge: Edges = warp.edges().intersection(&other.eadjs()).into_iter().cloned().collect::<Edges>();
                if !bridge.is_empty() {
                    let warp_e: Edge = bridge.drain().next().unwrap();
                    let mut weft_es: Edges = edge_adj.get(&warp_e).unwrap().intersection(&other.edges()).into_iter().cloned().collect::<Edges>();
                    if !weft_es.is_empty() {
                        let weft_e: Edge = weft_es.drain().next().unwrap();
                        warp.join(warp_e, weft_e, &mut other);
                        processed.extend([idx]);
                    }
                }
            }
        }
    }
    warp.retrieve()
}
    
fn warp_loom(v3verts: &Vectors3d, adj: &Adjacency, vert_idx: &VertIdx) -> Loom {
    let (z_adj, z_length) = shrink_adjacency(&v3verts, &adj);
    let spool: Spool = spool_yarn(&z_adj);
    let mut bobbins: Bobbins = Vec::new();
    let mut warps: Warps;
    let mut loom: Loom = Vec::new();
    for (zlevel, order) in z_length {
        let mut yarn: Yarn = spool[&(zlevel % 4 + 4).try_into().unwrap()].clone();
        yarn.slice_axis_inplace(Axis(0), Slice::new((yarn.len_of(Axis(0)) - order).try_into().unwrap(), None, 1));
        let node_yarn: Vec<u32> = yarn.outer_iter().map(|row| Vector3D::to_node(row[0], row[1], zlevel, &vert_idx)).collect();
        if bobbins.is_empty() { warps = vec![node_yarn] } else { warps = cut(node_yarn, &bobbins) }
        let mut woven: Processed = HashSet::new();
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
            loom.extend(vec![VecDeque::from(seq.iter().cloned().collect::<Thread>())]);
        }
        let v3verts: &Vectors3d = &vectorize(&VERTS);
        if zlevel != -1 { bobbins = wind(&mut loom, v3verts, &vert_idx) }
    }
    for thread in &mut loom {
        let nodes: Path = thread.iter().map(|&node| v3verts[node as usize].mirror_z(&vert_idx)).collect();
        thread.extend(nodes.into_iter().rev());
    }
    loom.sort_by_key(|w| w.len());
    loom
}

fn spool_yarn(z_adj: &Adjacency) -> Spool {
    let verts: &Verts2d = &VERTS.iter().clone().map(|&(x, y, _)| (x, y)).collect::<Verts2d>();
    let weights: Weights = make_weights(&z_adj, &VERTS);
    let natural: Yarn = convert_from_nodes(spin(&z_adj, &weights), &verts);
    let colored: Yarn = color(&natural);
    HashMap::from([(3, natural), (1, colored)])
}