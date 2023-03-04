use crate::graphs::data::g_16192::VERTS;
use crate::structs::cycle::Cycle;
use crate::types::types::{Adjacency, Done, Edge, EdgeAdjacency, Edges, Solution, WarpedLoom, Wefts, Vectors3d, VertIdx, VertsC3};

use super::warp::warp_loom;

pub fn weave(v3verts: &Vectors3d, adj: &Adjacency, vert_idx: &VertIdx, edge_adj: &EdgeAdjacency, verts: &VertsC3, var: &[[i32; 3]]) -> Solution {
    let mut warp_wefts: Wefts = warp_loom(v3verts, &adj, vert_idx, verts, var);
    let (warp, wefts) = warp_wefts.split_first_mut().unwrap();
    let warp: &mut Cycle = Cycle::new(warp, &adj, &edge_adj, VERTS, true);
    let loom: WarpedLoom = wefts.iter().enumerate().map(|(idx, seq)| (idx, Cycle::new(&seq, &adj, &edge_adj, VERTS, false))).collect();
    let mut done: Done = Done::new();
    let loom_order = loom.keys().len();
    if loom_order > 0 {
        'weaving: loop {
            for idx in loom.keys() {
                let done_len = done.len();
                if done_len == loom_order { break 'weaving }
                if done_len == loom_order - 1 { warp.set_last() }
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