use std::collections::{VecDeque, HashMap, HashSet};

use crate::structs::cycle::Cycle;
use crate::graph::types::{
    Adjacency, Done, EdgeAdjacency, Solution, Vectors3d, VertsC3, WarpedLoom, Wefts, Verts, Weights,
};

use super::warp::warp_loom;

pub fn weave(
    v3verts: &Vectors3d,
    adj: &Adjacency,
    vert_idx: &HashMap<(i32, i32, i32), u32>,
    edge_adj: &EdgeAdjacency,
    verts: &Verts,
    var: &Vec<[i32; 3]>,
    weights: &Weights,
    z_adj: &HashMap<u32, HashSet<u32>>,
    z_length: &Vec<(i32, usize)>
) -> Solution {
    let mut warp_wefts: Wefts = warp_loom(v3verts, vert_idx, verts, var, weights, z_adj, z_length);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut Cycle = Cycle::new(warp, &adj, &edge_adj, verts);
    join_loops(warp, wefts, adj, verts, edge_adj);
    warp.retrieve()
}

pub fn join_loops(
    warp: &mut Cycle,
    wefts: &mut [VecDeque<u32>],
    adj: &Adjacency,
    verts: &VertsC3,
    edge_adj: &EdgeAdjacency,
) {
    let loom: WarpedLoom = wefts
        .iter()
        .enumerate()
        .map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, &edge_adj, verts)))
        .collect();
    let mut done: Done = Done::new();
    let loom_order = loom.keys().len();
    if loom_order > 0 {
        loop {
            for idx in loom.keys() {
                if done.len() != loom_order {
                    if !done.contains(idx) {
                        let mut other: Cycle = loom[&*idx].clone();
                        if let Some(warp_e) = warp.edges().intersection(&other.eadjs()).next() {
                            if let Some(weft_e) = edge_adj
                                .get(&warp_e)
                                .unwrap()
                                .intersection(&other.edges())
                                .next()
                            {
                                warp.join(*warp_e, *weft_e, &mut other);
                                done.extend([idx])
                            }
                        }
                    }
                } else {
                    return;
                }
            }
        }
    }
}
