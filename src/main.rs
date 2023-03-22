/// see n_order.txt for a list of n and the corresponding order:
/// n: 100 = 1_373_600 vertices
/// ```
/// cargo run --release [N] [N_UPPER_INCLUSIVE]
/// cargo run --release 1 100
/// ```
/// builds binary under hamcycle/target/release/hamcycle
/// runs binary: ./hamcycle/target/release/hamcycle
/// starts with the first order in the sequence with 32 vertices,
/// creates the graph for that order and solves it,
/// continues to the next orders up to the 100th which is an order with 1,373,600 vertices,
/// makes graph, solves it
/// 1 (start with order 8 end at order 1,373,600) 100
/////////////////////////////////////////////////////////////////////////////
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
    let n_start: u32 = match args.get(1) {
        Some(arg) => {
            let parsed = arg.parse().unwrap_or(100);
            if parsed == 0 {
                1
            } else {
                parsed
            }
        }
        None => 100,
    };
    let n_end: u32 = match args.get(2) {
        Some(arg) => {
            let parsed = arg.parse().unwrap_or(n_start);
            if parsed < n_start {
                n_start
            } else {
                parsed
            }
        }
        None => n_start,
    };
    for level in n_start..=n_end {
        find_solution(level)?;
    }
    Ok(())
}

pub fn find_solution(level: u32) -> Result<Solution, &'static str> {
    println!("MAKE ▦   GRAPH ➤ ⌘ SOLVE ✌ GRAPH ➤ CERTIFY ☑ SOLUTION");
    println!("MAKING GRAPH....");
    let start: Instant = Instant::now();
    let (n, order, verts, vi_map, adj, z_adj, z_order, max_xyz) = make_graph(level);
    let dur_graph = Instant::now() - start;
    println!("MADE GRAPH: 🕗 {dur_graph:?}. SOLVING GRAPH ⭕️ {order}");
    let start: Instant = Instant::now();
    let solution = weave::weave(&adj, vi_map, verts, z_adj, z_order, max_xyz);
    let dur = Instant::now() - start;
    println!(
        "🇳 {n:>4} FINISHED WEAVE. NOW CERTIFYING... 🕗 {}",
        dur.as_secs_f32()
    );
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
