use std::env;
use std::time::Instant;

pub mod graph;

use graph::check;
use graph::make;
use graph::shrink;
use graph::solve;
use graph::types;
use graph::utils;

fn main() {
    // cargo run --release 1373600 10
    let args: Vec<String> = env::args().collect();
    weave_nodes(
        args
            .get(1)
            .unwrap_or(&"79040".to_string())
            .parse()
            .unwrap_or(79040), 
        args
            .get(2)
            .unwrap_or(&"100".to_string())
            .parse()
            .unwrap_or(10)
    )
}

pub fn weave_nodes(order: u32, repeats: u32) {
    println!(
        "MAKING GRAPH::: | ⭕️ ORDER: {:?} | REPEATS: {}",
        order, repeats
    );
    let max_xyz = utils::get_max_xyz(order as i32);
    let verts: types::Verts = make::vertices(max_xyz);
    let vi_map: types::VIMap = make::vi_map(&verts);
    let adj: types::Adjacency = make::adjacency(&verts, max_xyz, &vi_map);
    let edges: types::Edges = make::edges_from_adjacency(&adj);
    let edge_adj = make::edges_adjacency(&adj, &edges, &verts);
    let (z_adj, z_length) = shrink::adjacency(&verts, &adj);
    println!(
        "SOLVING::GRAPH:::⭕️ {:?} * {}", 
        order, repeats
    );
    let mut solution: types::Solution = types::Solution::new();

    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = solve::weave(&adj, &vi_map, &edge_adj, &verts, &z_adj, &z_length);
    }
    let dur = utils::elapsed_ms(start, Instant::now(), repeats, "WEAVE");
    
    let id: check::SequenceID = check::id_seq(&solution, &adj);
    assert_eq!(check::SequenceID::HamCycle, id);
    println!(
        "i32 ⭕️ ORDER: {:?} | ID: {} | LEN: {:?} | DUR: {:?}",
        order, id, solution.len(), dur
    );
}
