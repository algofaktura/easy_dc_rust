use crate::structs::cycle::Cycle;
use crate::types::types::{
    Adjacency, Done, EdgeAdjacency, Solution, Vectors3d, VertIdx, VertsC3, WarpedLoom, Wefts,
};

use super::warp::warp_loom;

pub fn weave(
    v3verts: &Vectors3d,
    adj: &Adjacency,
    vert_idx: &VertIdx,
    edge_adj: &EdgeAdjacency,
    verts: &VertsC3,
    var: &[[i32; 3]],
) -> Solution {
    let mut warp_wefts: Wefts = warp_loom(v3verts, &adj, vert_idx, verts, var);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut Cycle = Cycle::new(warp, &adj, &edge_adj, verts);
    let loom: WarpedLoom = wefts
        .iter()
        .enumerate()
        .map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, &edge_adj, verts)))
        .collect();
    join_loops(loom.keys().len(), warp, &loom, edge_adj);
    warp.retrieve()
}

pub fn join_loops(
    loom_order: usize,
    warp: &mut Cycle,
    loom: &WarpedLoom,
    edge_adj: &EdgeAdjacency,
) {
    let mut done: Done = Done::new();
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