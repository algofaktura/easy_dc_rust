extern crate fxhash;
/// see n_order.txt for a list of n and the corresponding order:
/// n: 100 = 1_373_600 vertices
/// ```
/// cargo run --release [N] [N_UPPER_INCLUSIVE] [REPEATS]
/// cargo run --release 1 100 10
/// ```
/// builds binary under hamcycle/target/release/hamcycle
/// runs binary: ./hamcycle/target/release/hamcycle
/// starts with the first order in the sequence with 32 vertices,
/// creates the graph for that order and solves it 10 times,
/// getting the best runtime for each order.
/// continues to the next orders up to the 100th which is an order with 1,373,600 vertices,
/// makes graph, solves it ten times....
/// 1 (start with order 8 end at order 1,373,600) 100 10 (solve graph 10 times for each order)
/////////////////////////////////////////////////////////////////////////////
extern crate rayon;

use std::{env, f32::INFINITY, time::Instant};

pub mod graph;
pub mod play;

use graph::{
    types::*,
    utils::certify::{self, SequenceID},
    weave,
};

use crate::graph::make::vert_neighs;

pub fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let n_start: u32 = args
        .get(1)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);
    let n_end: u32 = args
        .get(2)
        .unwrap_or(&"{n_start}".to_string())
        .parse()
        .unwrap_or(n_start);

    let repeats: u32 = args.get(3).unwrap_or(&"1".to_string()).parse().unwrap_or(1);

    for level in n_start..=n_end {
        let graph = graph::make::make_graphx(level);
        find_solutionx(graph, repeats)?;
    }
    Ok(())
}

pub fn find_solutionx(
    (n, order, vertn, vi_map, z_adj, z_order, max_xyz): (
        u32,
        u32,
        VertN,
        VIMap,
        Adjacency,
        ZOrder,
        i16,
    ),
    repeats: u32,
) -> Result<Solution, &'static str> {
    let mut min_dur = INFINITY;
    let mut solution = Solution::with_capacity(order as usize);
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave::weavex(&vertn, &vi_map, &z_adj, &z_order, max_xyz);
        let dur = (Instant::now() - start).as_secs_f32();
        if dur < min_dur {
            min_dur = dur
        }
    }
    let seq_id = certify::id_seqx(&solution, &vertn);
    println!("| 🇳 {n:>4} | ⭕️ {order:>10} | 🕗 {min_dur:>14.7} | 📌 {seq_id:?} |");
    assert_eq!(seq_id, SequenceID::HamCycle);
    Ok(solution)
}

pub fn find_solution(
    (n, order, verts, vi_map, adj, z_adj, z_order, max_xyz): (
        u32,
        u32,
        VertsVec,
        VIMap,
        Adjacency,
        Adjacency,
        ZOrder,
        i16,
    ),
    repeats: u32,
) -> Result<Solution, &'static str> {
    let vertn: Vec<(Vert, Neighbors)> = vert_neighs(&verts, max_xyz, &vi_map);

    test_access(order, &verts, &adj, &vertn);

    let mut min_dur = INFINITY;
    let mut solution = Solution::with_capacity(order as usize);
    let start: Instant = Instant::now();
    for _ in 0..repeats {
        solution = weave::weave(&adj, &vi_map, &verts, &z_adj, &z_order, max_xyz);
        let dur = (Instant::now() - start).as_secs_f32();
        if dur < min_dur {
            min_dur = dur
        }
    }
    let seq_id = certify::id_seq(&solution, &adj);
    println!("| 🇳 {n:>4} | ⭕️ {order:>10} | 🕗 {min_dur:>14.7} | 📌 {seq_id:?} |");
    assert_eq!(seq_id, SequenceID::HamCycle);
    Ok(solution)
}

pub fn test_access(order: u32, verts: &Verts, adj: &Adjacency, vertn: &Vec<(Vert, Neighbors)>) {
    // testing out how long it takes to access a vertn[0].0 for the vert and vertn[0].1 for the neighbors
    // vs. vert[0] and adj[0].
    // iterate through 0-order - 1
    let start: Instant = Instant::now();
    for i in 0..order as usize - 1 {
        let vert = verts[i.clone()];
        let neighs = &adj[&(i as u32)];
        if i == 0 {
            println!("VERT {:?}", vert);
            println!("NEIGHS {:?}", neighs);
        }
    }
    println!("NORMAL {}", (Instant::now() - start).as_secs_f32());

    let start: Instant = Instant::now();
    for i in 0..order as usize - 1 {
        let vert = vertn[i].0;
        let neighs = &vertn[order as usize - i - 1].1;
        if i == 0 {
            println!("VERT {:?}", vert);
            println!("NEIGHS {:?}", neighs);
        }
    }
    println!("VERTN {}", (Instant::now() - start).as_secs_f32());
}
