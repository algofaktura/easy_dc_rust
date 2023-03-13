extern crate rayon;

use std::env;
use std::time::Instant;

pub mod graph;

use graph::{
    check,
    types::*,
    weave
};

// pub fn solve_range() {
//     for n in 100..111 {
//         find_solution(make_graph(n), 1)
//     }
// }

pub fn main() {
    // (n, order): (1, 8), (2, 32)...(100, 1,373,600), (200, 10,827,200), (300, )
    // cargo run --release 100 10
    let args: Vec<String> = env::args().collect();
    let n: u32 = args
        .get(1)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);
    let repeats: u32 = args.get(2).unwrap_or(&"1".to_string()).parse().unwrap_or(1);
    let graph = graph::make::make_graph(n);
    find_solution(graph, repeats)
}

pub fn find_solution(
    (n, order, verts, vi_map, adj, edge_adj, z_adj, z_order): (
        u32,
        u32,
        Verts,
        VIMap,
        Adjacency,
        EdgeAdjacency,
        Adjacency,
        ZOrder,
    ),
    repeats: u32,
) {
    let mut solution: Solution = Solution::new();
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave::weave(&adj, &vi_map, &edge_adj, &verts, &z_adj, &z_order);
    }
    let dur = Instant::now() - start;
    let seq_id = check::id_seq(&solution, &adj);
    println!(
        "N: {:?} | ⭕️ ORDER: {:?} | REPS: {} | DUR: {} | ID: {:?}",
        n,
        order,
        repeats,
        dur.as_secs_f64(),
        seq_id,
    );
}
