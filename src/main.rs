use std::collections::HashMap;
use std::env;
use std::time::Instant;

pub mod graph;
pub mod structs;
pub mod utils;

// use graph::data::g_105952::{ADJ, EDGES, VAR, VERTS};
// use graph::check::{id_seq, SequenceID, SequenceID::HamCycle};
use graph::make::vertices::make_vertices;
use graph::make::vertices::make_vertices_i16;
// use graph::make::vert_idx::make_vi_mapping;
use graph::make::vert_idx::make_vi_mapping3;

use crate::graph::check::{SequenceID, id_seq};
use crate::graph::make::adjacency_edges_i16::get_adj_i16;
// use crate::graph::make::adjacency_edges::{get_adj_ref, get_edges_ref};
use crate::graph::make::adjacency_edges::{get_adj, get_edges};
// use crate::graph::make::edges_adjacency::make_edges_adj_ref2;
use crate::graph::make::edges_adjacency::make_edges_adj;
use crate::graph::make::edges_adjacency::make_edges_adj_i16;
use crate::graph::make::vert_idx::make_vi_mapping_i16;
use crate::graph::make::weights::make_weights;
use crate::graph::make::weights::make_weights_i16;
// use crate::graph::make::weights::make_weights_ref;
use crate::graph::measure::get_max_xyz;
// use crate::graph::measure::get_max_xyz_i16;

// use crate::graph::measure::get_max_xyz_i16;
use crate::graph::shrink::shrink_adjacency_2;
use crate::graph::shrink::shrink_adjacency_i16;
use crate::graph::solve::weave::weave;
use crate::graph::solve::weave_i16;
use crate::graph::translate::from_v3c_to_vect3d_i16;
use crate::graph::translate::from_verts_to_vertsc_i16;
// use crate::graph::solve::weave_ref::weave;
use crate::graph::translate::{from_v3c_to_vect3d, from_verts_to_vertsc};
// use crate::graph::types::{Vectors3d, AdjacencyRef, Verts};
use crate::graph::types::{Vectors3d, Adjacency, Verts, Edges, Solution};
// use crate::structs::vector::Vector3D;
// use graph::make::edges_adjacency::make_edges_adj;
// use graph::translate::{from_const_adj_to_adj, from_v3c_to_vect3d};
// use graph::types::*;

use utils::time::elapsed_ms;

// use solve::weave::weave;

fn main() {
    // cargo run --release 32 100
    let args: Vec<String> = env::args().collect();
    let order = args.get(1).unwrap_or(&"79040".to_string()).parse().unwrap_or(79040);
    let repeats = args.get(2).unwrap_or(&"100".to_string()).parse().unwrap_or(1000);

    solve_node_version(order, repeats);
    solve_node_version_i16(order, repeats);
}

pub fn solve_node_version(order: u32, repeats: u32) {
    println!("MAKING GRAPH::: | ⭕️ ORDER: {:?} | REPEATS: {}", order, repeats);
    let max_xyz = get_max_xyz(order as i32);
    let verts: Verts = make_vertices(max_xyz);
    let v3verts: Vectors3d = from_v3c_to_vect3d(&verts);
    let vert_idx: HashMap<(i32, i32, i32), u32> = make_vi_mapping3(&verts);
    // println!("MAX XYZ i32 {:?} i16 {:?} | V {:?}, V16 {:?} {:?}", max_xyz, max_xyz, verts, verts, verts.len());
    let adj: Adjacency = get_adj(&verts, max_xyz, &vert_idx);
    let edges: Edges = get_edges(&adj);
    let edge_adj = make_edges_adj(&adj, &edges, &verts);
    let (z_adj, z_length) = shrink_adjacency_2(&verts, &adj);
    let weights = make_weights(&z_adj, &verts);
    let var = from_verts_to_vertsc(&verts);

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

pub fn solve_node_version_i16(order: u32, repeats: u32) {
    println!("MAKING GRAPH::: | ⭕️ ORDER: {:?} | REPEATS: {}", order, repeats);
    let max_xyz = get_max_xyz(order as i32);
    let verts = make_vertices_i16(max_xyz as i16);
    let v3verts = from_v3c_to_vect3d_i16(&verts);
    let vert_idx: HashMap<(i16, i16, i16), u32> = make_vi_mapping_i16(&verts);
    let adj: Adjacency = get_adj_i16(&verts, max_xyz, &vert_idx);
    let edges: Edges = get_edges(&adj);
    let edge_adj = make_edges_adj_i16(&adj, &edges, &verts);
    let (z_adj, z_length) = shrink_adjacency_i16(&verts, &adj);
    let weights = make_weights_i16(&z_adj, &verts);
    let var = from_verts_to_vertsc_i16(&verts);

    println!("SOLVING::GRAPH:::⭕️ {:?} * {}", order, repeats);
    let start: Instant = Instant::now();
    for _ in 0..repeats - 1 {
        weave_i16::weave(&v3verts, &adj, &vert_idx, &edge_adj, &verts, &var, &weights, &z_adj, &z_length);
    }
    let solution: Solution = weave_i16::weave(&v3verts, &adj, &vert_idx, &edge_adj, &verts, &var, &weights, &z_adj, &z_length);
    let dur = elapsed_ms(start, Instant::now(), repeats, "WEAVE");    
    let id: SequenceID = id_seq(&solution, &adj);
    assert_eq!(SequenceID::HamCycle, id);
    println!("i16 ⭕️ ORDER: {:?} | ID: {} | LEN: {:?} | DUR: {:?}", order, id, solution.len(), dur);
}