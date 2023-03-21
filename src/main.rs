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
extern crate fxhash;
extern crate rayon;

use std::{env, time::Instant};

pub mod graph;

use graph::{
    defs::*,
    utils::certify::{self, SequenceID},
    utils::make::make_graph,
    weave,
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

    for level in n_start..=n_end {
        find_solution(level)?;
    }
    Ok(())
}

pub fn find_solution(level: u32) -> Result<Solution, &'static str> {
    println!("MAKE GRAPH | SOLVE GRAPH | CERTIFY SOLUTION");
    println!("MAKING GRAPH....");
    let start: Instant = Instant::now();
    let (n, order, verts, vi_map, adj, z_adj, z_order, max_xyz) = make_graph(level);
    let dur_graph = Instant::now() - start;
    println!("MADE GRAPH: 🕗 {dur_graph:?}. SOLVING GRAPH ⭕️ {order}");
    let start: Instant = Instant::now();
    let solution = weave::weave(&adj, vi_map, &verts, z_adj, z_order, max_xyz);
    let dur = Instant::now() - start;
    println!("FINISHED WEAVE. NOW CERTIFYING... 🕗 {}", dur.as_secs_f32());
    let start: Instant = Instant::now();
    let seq_id = certify::id_seq(&solution, &adj);
    let dur_certify = Instant::now() - start;
    println!(
        "| 🇳 {n:>4} | 🕗 MAKE: {} | ⭕️ {order:>10} | 🕗 SOLVE: {} | 📌 {seq_id:?} | 🕗 CERTIFY: {}",
        dur_graph.as_secs_f32(),
        dur.as_secs_f32(),
        dur_certify.as_secs_f32()
    );
    assert_eq!(seq_id, SequenceID::HamCycle);
    Ok(solution)
}
