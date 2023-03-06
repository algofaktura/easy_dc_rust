use std::env;
use std::time::Instant;

pub mod graphs;
pub mod operators;
pub mod structs;
pub mod types;
pub mod utils;

use graphs::info::certify::{id_seq, SequenceID, SequenceID::HamCycle};
use graphs::utils::map::map_graph;
use types::types::*;
use utils::time::elapsed_ms;

use crate::graphs::utils::make::{make_edges_adj, make_vi_mapping};
use crate::graphs::utils::map::vectorize;
use crate::operators::weave::weave;

/// a rust implementation of easy_dc solver originally in python.
/// 
/// ```
/// cargo run --release -- order=32 repeats=2
/// ```
/// 
/// need to get order from cli args using above
/// get the appropriate graph or import it.
/// 
fn main() {
    use graphs::data::g_10640::{ADJ, EDGES, VAR, VERTS};
    let args: Vec<String> = env::args().collect();
    let order: u32 = args.get(1).unwrap_or(&"209880".to_string()).parse().unwrap();
    let repeats: u32 = args.get(2).unwrap_or(&"1".to_string()).parse().unwrap();
    let adj: Adjacency = map_graph(&ADJ);
    let v3verts: Vectors3d = vectorize(&VERTS);
    let vert_idx: VertIdx = make_vi_mapping(&v3verts);
    let edge_adj: EdgeAdjacency =
        make_edges_adj(&adj, &EDGES.iter().cloned().collect::<Edges>(), VERTS);
    let mut solution: Solution = Solution::new();
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave(&v3verts, &adj, &vert_idx, &edge_adj, &VERTS, &VAR)
    }
    elapsed_ms(start, Instant::now(), repeats, "WEAVE");
    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(HamCycle, id);
    println!("{}", id);
    println!("⭕️ ORDER: {:?} | ID: {} | {:?}", order, id, solution.len());
}
