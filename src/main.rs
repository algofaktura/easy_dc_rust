/// see n_order.txt for a list of n and the corresponding order:
/// n: 100 = 1_373_600 vertices
/// ```
/// cargo run --release [N] [N_UPPER_INCLUSIVE]
/// cargo run --release 1 100
/// ```
/// builds binary under hamcycle/target/release/hamcycle
/// runs binary: ./hamcycle/target/release/hamcycle
/// starts with the first order in the sequence with 32 vertices,
/// creates the graph for that order and solves it,
/// continues to the next orders up to the 100th which is an order with 1,373,600 vertices,
/// makes graph, solves it
/// 1 (start with order 8 end at order 1,373,600) 100
/////////////////////////////////////////////////////////////////////////////
extern crate rayon;

use std::{env, iter::zip, time::Instant};

pub mod graph;

use graph::{
    defs::*,
    utils::certify::{self, SequenceID},
    utils::make::make_graph,
    weave,
};

use crate::graph::utils::make::make_graph2;

pub fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let n_start: u32 = match args.get(1) {
        Some(arg) => match arg.parse::<u32>() {
            Ok(parsed) => {
                if parsed == 0 {
                    1
                } else {
                    parsed
                }
            }
            Err(_) => 100,
        },
        None => 100,
    };
    let n_end: u32 = match args.get(2) {
        Some(arg) => match arg.parse::<u32>() {
            Ok(parsed) => {
                if parsed < n_start {
                    n_start
                } else {
                    parsed
                }
            }
            Err(_) => n_start,
        },
        None => n_start,
    };
    for level in n_start..=n_end {
        find_solution(level, true)?;
    }
    Ok(())
}

pub fn find_solution(level: u32, _certify: bool) -> Result<Solution, &'static str> {
    println!("👷 MAKE GRAPH ➤ 🔀 SOLVE GRAPH ➤ 🔎 CERTIFY SOLUTION");

    println!("🛠️ MAKING GRAPH....");
    let mut start: Instant = Instant::now();
    let (n, order, z_adj, z_order, min_xyz) = make_graph(level);
    let dur_make = Instant::now() - start;
    println!("MADE GRAPH: 🕗 {dur_make:?}. 🔀 SOLVING GRAPH ⭕️ {order}");
    start = Instant::now();
    let solution = weave::weave(z_adj, z_order, min_xyz, order);
    let dur_solve = Instant::now() - start;
    println!(
        "| 🇳 {n:>4} | ⭕️ {order:>10} | 🕗 SOLVE: {} |",
        dur_solve.as_secs_f32()
    );

    if _certify {
        let adj = make_graph2(n);
        println!("🇳 {n:>4} FINISHED WEAVING. 🔎 CERTIFYING SOLUTION...");
        start = Instant::now();
        let seq_id = certify::id_seq(&solution, &adj);
        let dur_certify = Instant::now() - start;
        println!(
        "| 🇳 {n:>4} | 🕗 MAKE: {} | ⭕️ {order:>10} | 🕗 SOLVE: {} | 📌 {seq_id:?} | 🕗 CERTIFY: {}",
        dur_make.as_secs_f32(),
        dur_solve.as_secs_f32(),
        dur_certify.as_secs_f32()
        );
        assert_eq!(seq_id, SequenceID::HamCycle);
    }
    Ok(solution)
}

pub fn get_zorders(n: usize) -> Vec<usize> {
    (1..=n).map(|_n| 2 * _n * (_n + 1)).collect()
}

pub fn get_zlevels(max_xyz: i16) -> Vec<i16> {
    (-max_xyz..=-1).step_by(2).collect()
}

pub fn get_max_xyz(order: u32) -> SignedIdx {
    (get_n_from_order(order) * 2 - 1) as i32
}

pub fn get_max_xyzn(n: u32) -> SignedIdx {
    (n * 2 - 1) as i32
}

pub fn get_order_from_n(n: u32) -> u32 {
    ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as u32
}

pub fn get_n_from_order(order: u32) -> u32 {
    (((3.0 / 4.0) * order as f64).powf(1.0 / 3.0) - 2.0 / 3.0).round() as u32
}

pub fn make_zorders(n: usize) -> Vec<(i16, usize)> {
    zip(
        (-((n * 2 - 1) as i16)..=-1).step_by(2),
        (1..=n).map(|_n| 2 * _n * (_n + 1)),
    )
    .collect()
}
