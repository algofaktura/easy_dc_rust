use std::collections::HashMap;
use std::env;
use std::time::Instant;

pub mod graph;

use graph::check::{id_seq, SequenceID};
use graph::make::{
    make_adjacency, make_edges_adjacency, make_edges_from_adjacency, make_vertices, make_vert_idx, make_weights,
};

use graph::shrink::shrink_adjacency;
use graph::solve::weave;
use graph::types::{Adjacency, Edges, Solution, Verts};
use graph::utils::{elapsed_ms, get_max_xyz};

fn main() {
    // cargo run --release 32 100
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
    solve_node_version(order, repeats);
}

pub fn solve_node_version(order: u32, repeats: u32) {
    println!(
        "MAKING GRAPH::: | ⭕️ ORDER: {:?} | REPEATS: {}",
        order, repeats
    );
    let max_xyz = get_max_xyz(order as i32);
    let verts: Verts = make_vertices(max_xyz);
    let vert_idx: HashMap<(i32, i32, i32), u32> = make_vert_idx(&verts);
    let adj: Adjacency = make_adjacency(&verts, max_xyz, &vert_idx);
    let edges: Edges = make_edges_from_adjacency(&adj);
    let edge_adj = make_edges_adjacency(&adj, &edges, &verts);
    let (z_adj, z_length) = shrink_adjacency(&verts, &adj);
    let weights = make_weights(&z_adj, &verts);
    println!(
        "MAX XYZ i32 {:?} | len VERTS{:?} VI{:?} ADJ{:?}, EA{:?}",
        max_xyz,
        verts.len(),
        vert_idx.len(),
        adj.len(),
        edge_adj.len(),
    );

    println!("SOLVING::GRAPH:::⭕️ {:?} * {}", order, repeats);
    let start: Instant = Instant::now();
    for _ in 0..repeats - 1 {
        weave(
            &adj, &vert_idx, &edge_adj, &verts, &weights, &z_adj, &z_length,
        );
    }
    let solution: Solution = weave(
        &adj, &vert_idx, &edge_adj, &verts, &weights, &z_adj, &z_length,
    );
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
