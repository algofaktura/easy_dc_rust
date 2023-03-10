use std::collections::HashMap;
use std::env;
use std::time::Instant;

pub mod compare;
pub mod graph;
pub mod structs;
pub mod utils;

use graph::make::vertices::make_vertices;
use graph::make::vert_idx::make_vi_mapping;
use graph::check::{SequenceID, id_seq};
use graph::make::adjacency_edges::{make_adj, make_edges_from_adj};
use graph::make::edges_adjacency::make_edges_adj;
use graph::make::weights::make_weights;
use graph::measure::get_max_xyz;
use graph::shrink::shrink_adjacency_2;
use graph::solve::weave::weave;
use graph::translate::{from_v3c_to_vect3d, from_verts_to_vertsc};
use graph::types::{Vectors3d, Adjacency, Verts, Edges, Solution};

use utils::time::elapsed_ms;

fn main() {
    // cargo run --release 32 100
    let args: Vec<String> = env::args().collect();
    let order = args.get(1).unwrap_or(&"79040".to_string()).parse().unwrap_or(79040);
    let repeats = args.get(2).unwrap_or(&"100".to_string()).parse().unwrap_or(10);
    solve_node_version(order, repeats);
}

pub fn solve_node_version(order: u32, repeats: u32) {
    println!("MAKING GRAPH::: | ⭕️ ORDER: {:?} | REPEATS: {}", order, repeats);
    let max_xyz = get_max_xyz(order as i32);
    let verts: Verts = make_vertices(max_xyz);
    let v3verts: Vectors3d = from_v3c_to_vect3d(&verts);
    let vert_idx: HashMap<(i32, i32, i32), u32> = make_vi_mapping(&verts);
    let adj: Adjacency = make_adj(&verts, max_xyz, &vert_idx);
    let edges: Edges = make_edges_from_adj(&adj);
    let edge_adj = make_edges_adj(&adj, &edges, &verts);
    let (z_adj, z_length) = shrink_adjacency_2(&verts, &adj);
    let weights = make_weights(&z_adj, &verts);
    let var = from_verts_to_vertsc(&verts);
    println!("MAX XYZ i32 {:?} | len VERTS{:?} VI{:?} ADJ{:?}, EA{:?} VAR{:?}", max_xyz, verts.len(), vert_idx.len(), adj.len(), edge_adj.len(), var.len());

    println!("SOLVING::GRAPH:::⭕️ {:?} * {}", order, repeats);
    let start: Instant = Instant::now();
    for _ in 0..repeats - 1 {
        weave(&v3verts, &adj, &vert_idx, &edge_adj, &verts, &var, &weights, &z_adj, &z_length);
    }
    let solution: Solution = weave(&v3verts, &adj, &vert_idx, &edge_adj, &verts, &var, &weights, &z_adj, &z_length);
    let dur = elapsed_ms(start, Instant::now(), repeats, "WEAVE");    
    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(SequenceID::HamCycle, id);
    println!("i32 ⭕️ ORDER: {:?} | ID: {} | LEN: {:?} | DUR: {:?}", order, id, solution.len(), dur);
}
