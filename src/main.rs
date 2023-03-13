extern crate rayon;
extern crate serde_json;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::time::Instant;

pub mod graph;

use graph::check;
use graph::make;
use graph::shrink;
use graph::types::*;
use graph::utils;
use graph::utils::get_order_from_n;
use graph::weave;

// use crate::graph::utils::path_as_absumv;

fn main() {
    // cargo run --release 1373600 10
    let args: Vec<String> = env::args().collect();
    find_solution(make_graph(
            args.get(1)
                .unwrap_or(&"100".to_string())
                .parse()
                .unwrap_or(100),
            args.get(2)
                .unwrap_or(&"1".to_string())
                .parse()
                .unwrap_or(1)
    ))
}

pub fn find_solution(
    (order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order): (
        u32,
        u32,
        Verts,
        VIMap,
        Adjacency,
        EdgeAdjacency,
        Adjacency,
        ZOrder,
    ),
) {
    let mut solution: Solution = Solution::new();
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave::weave(&adj, &vi_map, &edge_adj, &verts, &z_adj, &z_order);
    }
    // println!("{:?}", path_as_absumv(&solution, &verts));
    println!(
        "⭕️ ORDER: {:?} | REPS: {} | DUR: {} | ID: {:?}",
        order,
        repeats,
        utils::elapsed_ms(start, Instant::now(), repeats, "WEAVE"),
        check::id_seq(&solution, &adj),
    );
}

pub fn make_graph(
    n: u32,
    repeats: u32,
) -> (
    u32,
    u32,
    Verts,
    VIMap,
    Adjacency,
    EdgeAdjacency,
    Adjacency,
    ZOrder,
) {

    let order = get_order_from_n(n as i32);
    let max_xyz = utils::get_max_xyz(order as i32) as i16;
    let verts: Verts = make::vertices(max_xyz);
    println!("MAX XYZ {max_xyz} | VERTS LEN: {:?} | REPEATS: {repeats} | N: {n}", verts.len());
    let vi_map: VIMap = make::vi_map(&verts);
    let adj: Adjacency = make::adjacency_map(&verts, max_xyz, &vi_map);
    let edge_adj: HashMap<(u32, u32), HashSet<(u32, u32)>> =
        make::edges_adjacency_mapping(&adj, &verts);
    let (z_adj, z_order) = shrink::adjacency(&verts, &adj);
    (order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order)
}
