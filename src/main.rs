extern crate rayon;
extern crate serde_json;

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

fn main() {
    for n in 100..111 {
        find_solution(make_graph(n, 1))
    }
}

pub fn solve_from_args() {
    // (n, order): (1, 8), (2, 32)...(100, 1,373,600), (200, 10,827,200), (300, )
    // cargo run --release 100 10
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

pub fn make_graph(
    n: u32,
    repeats: u32,
) -> (
    u32,
    u32,
    u32,
    Verts,
    VIMap,
    Adjacency,
    EdgeAdjacency,
    Adjacency,
    ZOrder,
) {
    let order = get_order_from_n(n);
    let max_xyz = utils::get_max_xyz(order as i32);
    let verts: Verts = make::vertices(max_xyz);
    let vi_map: VIMap = make::vi_map(&verts);
    let adj: Adjacency = make::adjacency_map(&verts, max_xyz, &vi_map);
    let edge_adj: EdgeAdjacency =
        make::edges_adjacency_mapping(&adj, &verts);
    let (z_adj, z_order) = shrink::adjacency(&verts, &adj);
    (n, order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order)
}

pub fn find_solution(
    (n, order, repeats, verts, vi_map, adj, edge_adj, z_adj, z_order): (
        u32,
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
