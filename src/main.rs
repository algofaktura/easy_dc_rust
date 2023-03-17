/// see n_order.txt for a list of n and the corresponding order:
/// n: 100 = 1_373_600 vertices
/// ```
/// cargo run --release [N] [N_UPPER_INCLUSIVE] [REPEATS]
/// cargo run --release 1 100 10
/// ```
/// builds binary under hamcycle/target/release/hamcycle
/// runs binary: ./hamcycle/target/release/hamcycle
/// starts with the first order in the sequence with 32 vertices,
/// creates the graph for that order and solves it 10 times,
/// getting the best runtime for each order.
/// continues to the next orders up to the 100th which is an order with 1,373,600 vertices,
/// makes graph, solves it ten times....
/// 1 (start with order 8 end at order 1,373,600) 100 10 (solve graph 10 times for each order)
/////////////////////////////////////////////////////////////////////////////
extern crate rayon;

use std::{
    env, 
    f32::INFINITY, 
    time::Instant
};

pub mod graph;

use graph::{
    types::*,
    utils::certify::{self, SequenceID},
    weave
};

pub fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let n_start: u32 = args
        .get(1)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);
    let n_end: u32 = args
        .get(2)
        .unwrap_or(&"{n_start}".to_string())
        .parse()
        .unwrap_or(n_start);

    let repeats: u32 = args.get(3).unwrap_or(&"1".to_string()).parse().unwrap_or(1);

    for level in n_start..=n_end {
        find_solution(graph::make::make_graph(level), repeats)?
    }
    Ok(())
}

pub fn find_solution(
    (n, order, verts, vi_map, adj, edge_adj, z_adj, z_order, max_xyz): (
        u32,
        u32,
        VertsVec,
        VIMap,
        Adjacency,
        EdgeAdjacency,
        Adjacency,
        ZOrder,
        i32,
    ),
    repeats: u32,
) -> Result<(), &'static str> {
    let mut min_dur = INFINITY;
    let mut solution = Solution::with_capacity(order as usize);
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave::weave(&adj, &vi_map, &edge_adj, &verts, &z_adj, &z_order, max_xyz);
        let dur = (Instant::now() - start).as_secs_f32();
        if min_dur > dur {
            min_dur = dur
        }
    }
    let seq_id = certify::id_seq(&solution, &adj);
    println!("| 🇳 {n:>4} | ⭕️ {order:>10} | 🕗 {min_dur:>14.7} | 📌 {seq_id:?} |");
    assert_eq!(seq_id, SequenceID::HamCycle);
    Ok(())
}
