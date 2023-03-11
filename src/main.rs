use std::env;
use std::time::Instant;

pub mod graph;

use graph::check::{id_seq, SequenceID};
use graph::make::{
    make_adjacency, make_edges_adjacency, make_edges_from_adjacency, make_vertices, make_vi_map,
};
use graph::shrink::shrink_adjacency;
use graph::solve::weave;
use graph::types::{Adjacency, Edges, Solution, Verts};
use graph::utils::{elapsed_ms, get_max_xyz};

use crate::graph::types::VIMap;

fn main() {
    // cargo run --release 1373600 10
    // run ⭕️rder: 1373600 ten times.
    let args: Vec<String> = env::args().collect();
    let order = args
        .get(1)
        .unwrap_or(&"79040".to_string())
        .parse()
        .unwrap_or(79040);
    let repeats = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(10);
    weave_nodes(order, repeats);
}

pub fn weave_nodes(order: u32, repeats: u32) {
    println!(
        "MAKING GRAPH::: | ⭕️ ORDER: {:?} | REPEATS: {}",
        order, repeats
    );
    let max_xyz = get_max_xyz(order as i32);
    let verts: Verts = make_vertices(max_xyz);
    let vi_map: VIMap = make_vi_map(&verts);
    let adj: Adjacency = make_adjacency(&verts, max_xyz, &vi_map);
    let edges: Edges = make_edges_from_adjacency(&adj);
    let edge_adj = make_edges_adjacency(&adj, &edges, &verts);
    let (z_adj, z_length) = shrink_adjacency(&verts, &adj);
    println!(
        "MAX XYZ i32 {:?} | len VERTS{:?} VI{:?} ADJ{:?}, EA{:?}",
        max_xyz,
        verts.len(),
        vi_map.len(),
        adj.len(),
        edge_adj.len(),
    );

    println!("SOLVING::GRAPH:::⭕️ {:?} * {}", order, repeats);
    let mut solution: Solution = Solution::new();
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave(&adj, &vi_map, &edge_adj, &verts, &z_adj, &z_length);
    }
    let dur = elapsed_ms(start, Instant::now(), repeats, "WEAVE");
    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(SequenceID::HamCycle, id);
    println!(
        "i32 ⭕️ ORDER: {:?} | ID: {} | LEN: {:?} | DUR: {:?}",
        order,
        id,
        solution.len(),
        dur
    );
}
