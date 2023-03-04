use std::time::Instant;

pub mod graphs;
pub mod operators;
pub mod structs;
pub mod types;
pub mod utils; 

use graphs::data::g_16192::{ADJ, EDGES, VERTS, VAR};
use graphs::info::certify::{id_seq, SequenceID, SequenceID::HamCycle};
use graphs::utils::map::map_graph;
use operators::solve::solve;
use types::types::*;
use utils::time::elapsed_ms;

use crate::graphs::utils::make::{make_vi_mapping, make_edges_adj};
use crate::graphs::utils::map::vectorize;

const REPEATS: u32 = 1;

fn main() {
    let order: u32 = 16192;
    let adj: Adjacency = map_graph(&ADJ);
    let v3verts: Vectors3d = vectorize(&VERTS);
    let vert_idx: VertIdx = make_vi_mapping(&v3verts);
    let edge_adj: EdgeAdjacency = make_edges_adj(&adj, &EDGES.iter().cloned().collect::<Edges>());
    let mut solution: Solution = Solution::new();
    let start: Instant = Instant::now();
    for _ in 0..REPEATS { 
        solution = solve(order, &adj, &v3verts, &vert_idx, &edge_adj, &VERTS, VAR) 
    }
    elapsed_ms(start, Instant::now(), REPEATS, "WEAVE");

    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(HamCycle, id);
    println!("{:?}", id);
    println!("⭕️ ORDER: {:?} | ID: {:?} | {:?}", order, id, solution.len());
}