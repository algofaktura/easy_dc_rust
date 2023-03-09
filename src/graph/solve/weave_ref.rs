use std::collections::{HashSet, HashMap};

use crate::structs::cycle_ref::Cycle;
use crate::graph::types::{
    AdjacencyRef, Done, WarpedLoomRef,
};

use super::warp_ref::prepare_loom; 

pub fn weave<'a>(
    adj: &'a AdjacencyRef,
    vert_idx: &'a HashMap<(i32, i32, i32), &'a (i32, i32, i32)>,
    edge_adj: &'a HashMap<&'a (&'a (i32, i32, i32), &'a (i32, i32, i32)), HashSet<&'a (&'a (i32, i32, i32), &'a (i32, i32, i32))>>,
    verts: &'a Vec<(i32, i32, i32)>,
) -> Vec<(i32, i32, i32)> {
    let mut warp_wefts = prepare_loom(adj, vert_idx, verts);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut Cycle = Cycle::new(warp, &adj, verts);
    let loom: WarpedLoomRef = wefts
        .iter()
        .enumerate()
        .map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, verts)))
        .collect();
    let mut done: Done = Done::new();
    let loom_order = loom.keys().len();
    if loom_order > 0 {
        'weaving: loop {
            for idx in loom.keys() {
                if done.len() != loom_order {
                    if !done.contains(idx) {
                        let mut other: Cycle = loom[&*idx].clone();
                        if let Some(warp_e) = warp.get_edges().intersection(&other.get_eadjs(&edge_adj)).next() {
                            if let Some(weft_e) = edge_adj
                                .get(warp_e)
                                .unwrap()
                                .intersection(&other.get_edges())
                                // the problem is that edges_adjacency needs to be &EdgeRef, HashSet<&EdgeRef>
                                .next()
                            {
                                warp.join(*warp_e, &weft_e, &mut other);
                                done.extend([idx])
                            }
                        }
                    }
                } else {
                    break 'weaving;
                }
            }
        }
    }
    warp.retrieve()
}
