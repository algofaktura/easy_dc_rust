extern crate serde_json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::time::Instant;

pub mod graph;

use graph::check;
use graph::io::deserialize_data;
use graph::make;
use graph::shrink;
use graph::weave;
use graph::types::*;
use graph::utils;

use crate::graph::io::serialize_data;

fn main() {
    // cargo run --release 1373600 10
    let args: Vec<String> = env::args().collect();
    find_solution(make_graph(
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
    ))
}

pub fn make_graph(order: u32, repeats: u32) -> (
    u32,
    u32,
    Verts,
    VIMap,
    Adjacency,
    EdgeAdjacency,
    Adjacency,
    ZOrder
) 
{
    let max_xyz = utils::get_max_xyz(order as i32);
    let verts: Vec<(i32, i32, i32)> = make::vertices(max_xyz);
    let vi_map: HashMap<(i32, i32, i32), u32> = make::vi_map(&verts);
    let adj: HashMap<u32, HashSet<u32>> = make::adjacency_map(&verts, max_xyz, &vi_map);
    let edge_adj: HashMap<(u32, u32), HashSet<(u32, u32)>> = make::edges_adjacency_mapping(&adj, &verts);
    let (z_adj, z_order) = shrink::adjacency(&verts, &adj);
    (order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order)
}

pub fn find_solution(( 
    order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order 
): ( 
    u32, u32, Verts, VIMap, Adjacency, EdgeAdjacency, Adjacency, ZOrder 
)){
    let mut solution: Solution = Solution::new();
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave::weave(&adj, &vi_map, &edge_adj, &verts, &z_adj, &z_order);
    }
    println!(
        "⭕️ ORDER: {:?} | REPS: {} | DUR: {} | ID: {:?}",
        order, 
        repeats,
        utils::elapsed_ms(start, Instant::now(), repeats, "WEAVE"), 
        check::id_seq(&solution, &adj), 
    );
}

pub fn serialize_graph(order: u32) {
    let fpath = "/home/rommelo/Repos/RustRepos/hamcycle/src";
    let max_xyz = utils::get_max_xyz(order as i32);
    let verts: Vec<(i32, i32, i32)> = make::vertices(max_xyz);
    let vi_map: HashMap<(i32, i32, i32), u32> = make::vi_map(&verts);
    let adj: HashMap<u32, HashSet<u32>> = make::adjacency_map(&verts, max_xyz, &vi_map);
    let edge_adj: HashMap<(u32, u32), HashSet<(u32, u32)>> = make::edges_adjacency_mapping(&adj, &verts);
    let (z_adj, z_order) = shrink::adjacency(&verts, &adj);

    let _serialized = match serialize_data(fpath, verts, vi_map, adj, edge_adj, z_adj, z_order) {
        Ok(file) => file,
        Err(error) => panic!("couldn't serialize file {:?}", error),
    };

    let _deserialized = match deserialize_data(order, fpath) {
        Ok(file) => file,
        Err(error) => panic!("couldn't deserialize file {:?}", error),
    };
    println!("{:?}", _deserialized);
}