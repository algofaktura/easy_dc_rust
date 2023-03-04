use ndarray::{Axis, Slice};
use std::collections::{HashMap, HashSet, VecDeque};

pub mod graphs;
pub mod operators;
pub mod structs;
pub mod types;
pub mod utils;
pub mod solver;

use graphs::data::g_448::{VERTS, ADJ, EDGES, VAR};
use graphs::utils::make::{make_weights, make_vi_mapping, make_edges_adj};
use graphs::utils::map::{map_graph, vectorize, convert_from_nodes};
use graphs::utils::shrink::shrink_adjacency;
use graphs::info::certify::{id_seq, SequenceID, SequenceID::HamCycle};
use operators::color::color;
use operators::cut::cut;
use operators::spin::spin;
use operators::wind::wind;
use structs::vector::Vector3D;
use structs::cycle::Cycle;

use types::types::*;

use std::time::Instant;
use utils::time::elapsed_ms;

const REPEATS: u32 = 1;

fn main() {
    let adj: Adjacency = map_graph(&ADJ);
    let v3verts: Vectors3d = vectorize(&VERTS);
    let vert_idx: VertIdx = make_vi_mapping(&v3verts);
    let edge_adj: EdgeAdjacency = make_edges_adj(&adj, &EDGES.iter().cloned().collect::<Edges>());

    let start: Instant = Instant::now();
    let solution = weave(&v3verts, &adj, &vert_idx, &edge_adj);
    elapsed_ms(start, Instant:: now(), REPEATS, "weave");

    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(HamCycle, id);
    println!("{:?}", id);
    println!("⭕️ ORDER: {:?} | ID: {:?} | {:?}", ADJ.len(), id, solution.len());
}

fn weave(v3verts: &Vectors3d, adj: &Adjacency, vert_idx: &VertIdx, edge_adj: &EdgeAdjacency) -> Solution {
    let mut warp_wefts: Wefts = warp_loom(v3verts, &adj, vert_idx);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut Cycle = Cycle::new(warp, &adj, &edge_adj, VERTS, true);
    let loom: WarpedLoom = wefts.iter().enumerate().map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, &edge_adj, VERTS, false))).collect();
    let mut done: Done = HashSet::new();
    if loom.keys().len() > 0 {
        'weaving: loop {
            for idx in loom.keys() {
                if done.len() == loom.keys().len() { break 'weaving }
                if done.len() == loom.keys().len() - 1 { warp.set_last() }
                if done.contains(idx) { continue }
                let mut other: Cycle = loom[&*idx].clone();
                let mut bridge: Edges = warp.edges().intersection(&other.eadjs()).into_iter().cloned().collect::<Edges>();
                if !bridge.is_empty() {
                    let warp_e: Edge = bridge.drain().next().unwrap();
                    let mut weft_es: Edges = edge_adj.get(&warp_e).unwrap().intersection(&other.edges()).into_iter().cloned().collect::<Edges>();
                    if !weft_es.is_empty() {
                        warp.join(warp_e, weft_es.drain().next().unwrap(), &mut other);
                        done.extend([idx]);
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
        let mut woven: Vec<usize> = Vec::new();
        for thread in &mut loom {
            for (idx, warp) in warps.iter().enumerate() {
                if !woven.contains(&idx) {
                    for end in vec![0 as usize, thread.len() - 1] {
                        if thread[end] == warp[0 as usize] {
                            woven.extend([idx]);
                            if end == 0 as usize { 
                                warp[1..].iter().map(|item| thread.push_front(*item)).fold((), |_, _| ());
                            } else { 
                                thread.extend(&warp[1..]) 
                            }
                        }
                    }
                }
            }
        }
        for (_, seq) in warps.iter().enumerate().filter(|(idx, _)| !woven.contains(idx)) {
            loom.extend(vec![VecDeque::from(seq.iter().cloned().collect::<Thread>())]);
        }
        if zlevel == -1 { break }
        bobbins = wind(&mut loom, &vectorize(&VERTS), &vert_idx);
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
    let path = spin(&z_adj, &weights, &VAR);
    let natural: Yarn = convert_from_nodes(path, &verts);
    let colored: Yarn = color(&natural);
    HashMap::from([(3, natural), (1, colored)])
}