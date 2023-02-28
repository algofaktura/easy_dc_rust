pub mod data;
pub mod enums;
pub mod graphs;
pub mod info;
pub mod macros;
pub mod operators;
pub mod types;
pub mod utils;

pub mod structs;

use std::collections::{HashMap, HashSet, VecDeque};
use ndarray::Array2;
use structs::vector2d::color;
use std::time::Instant;

use crate::data::adjacencies::adj32;
use crate::data::vertices::verts32::VERTS_32;
use crate::graphs::stratify::shrink_adjacency;
use crate::graphs::make_weights::make_weights;
use crate::graphs::translate::{graph_to_map, translate_verts_3d, make_vi_mapping};
use crate::info::certify::SequenceID::HamCycle;
use crate::info::certify::id_seq;
use crate::operators::cut::cut;
use crate::operators::spin::spin;
use crate::operators::wind::wind;
use crate::structs::vector2d::{translate_from_nodes, reflect, shift};
use crate::structs::vector3d::Vector3D;
use crate::utils::time::elapsed_ms;

use crate::graphs::graph32::GRAPH;


use crate::types::types::Adj;

const REPEATS: u32 = 1_000_000;

fn main() {
    let verts = &VERTS_32.iter().clone().map(|&(x, y, _)| (x, y)).collect::<Vec<_>>();
    let v3verts: &Vec<Vector3D> = &translate_verts_3d(&VERTS_32);
    let adj: HashMap<u32, HashSet<u32>> = graph_to_map(&GRAPH);
    let vert_idx: HashMap<&Vector3D, u32> = make_vi_mapping(v3verts);
    let (z_adj, z_length) = shrink_adjacency(v3verts, &adj);
    println!("zadj {:?} z_length {:?}", z_adj, z_length);
    println!("{:?}", vert_idx);

    let weights: HashMap<u32, i32> = make_weights(&z_adj, &VERTS_32);

    // spool yarn
    let path: Vec<u32> = spin(&z_adj, &weights);

    let seq: [u32; 12] = path.iter().map(|&x| x as u32).collect::<Vec<u32>>().try_into().unwrap();
    assert_eq!(HamCycle, id_seq(&seq, &adj));

    // yarn needs to be woven together: turned to vectors and colored
    let natural: Array2<i32> = translate_from_nodes(path, &verts);
    let colored: Array2<i32> = color(&natural);


    println!("NATURAL {:?} | COLORED: {:?}", natural, colored);

    test_cut();
    test_make_vi();
    test_reflect_shift();
    test_edges();
}


fn test_cut() {
    let tour: Vec<u32> = vec![780, 778, 540, 610, 414, 5, 30, 406, 596, 516, 746, 730, 512, 576, 382, 498, 374, 562, 488, 706, 708, 490, 564, 376, 500, 384, 578, 514, 740, 756, 518, 598, 408, 532, 416, 612, 542, 346, 344, 256, 294, 246, 334, 326, 238, 286, 228, 316, 318, 230, 288, 240, 328, 336, 248, 296, 258, 190, 188, 176, 178];
    let subset: Vec<u32> = vec![416, 514, 258, 230, 542, 190];
    let expected: Vec<Vec<u32>> = vec![vec![514, 578, 384, 500, 376, 564, 490, 708, 706, 488, 562, 374, 498, 382, 576, 512, 730, 746, 516, 596, 406, 30, 5, 414, 610, 540, 778, 780], vec![416, 532, 408, 598, 518, 756, 740], vec![542, 612], vec![230, 318, 316, 228, 286, 238, 326, 334, 246, 294, 256, 344, 346], vec![258, 296, 248, 336, 328, 240, 288], vec![190, 188, 176, 178]];
    let result: Vec<Vec<u32>> = cut(tour.clone(), subset.clone());
    assert_eq!(expected, result);

    let start: Instant = Instant::now();
    for _i in 0..=REPEATS {
        let _result: Vec<Vec<u32>> = cut(tour.clone(), subset.clone());
    }
    elapsed_ms(start, Instant:: now(), REPEATS, "cut");
}

fn test_make_vi() {
    let verts32: &[(i32, i32, i32)] = &[(-1, -1, -1), (-1, -1, 1), (-1, 1, -1), (-1, 1, 1), (1, -1, -1), (1, -1, 1), (1, 1, -1), (1, 1, 1), (-3, -1, -1), (-3, -1, 1), (-3, 1, -1), (-3, 1, 1), (-1, -3, -1), (-1, -3, 1), (-1, -1, -3), (-1, -1, 3), (-1, 1, -3), (-1, 1, 3), (-1, 3, -1), (-1, 3, 1), (1, -3, -1), (1, -3, 1), (1, -1, -3), (1, -1, 3), (1, 1, -3), (1, 1, 3), (1, 3, -1), (1, 3, 1), (3, -1, -1), (3, -1, 1), (3, 1, -1), (3, 1, 1)];
    let v3verts: &Vec<Vector3D> = &translate_verts_3d(verts32);
    let vert_idx: HashMap<&Vector3D, u32> = make_vi_mapping(v3verts);
    let loom: Vec<VecDeque<u32>> = vec![
        VecDeque::from(vec![24, 22, 14, 16]),
        VecDeque::from(vec![18, 26, 6, 24, 22, 14, 16, 2, 10, 8, 0, 12, 20, 4, 28, 30]),
    ];
    let start: Instant = Instant::now();
    let mut test_loom = loom.clone();
    let bobbins: HashSet<u32> = wind(&mut test_loom, v3verts, &vert_idx);
    assert_eq!(bobbins, HashSet::from([19, 6, 31, 2]));
    assert_eq!(test_loom, vec![VecDeque::from([6, 24, 22, 14, 16, 2]), VecDeque::from([19, 18, 26, 6, 24, 22, 14, 16, 2, 10, 8, 0, 12, 20, 4, 28, 30, 31])]);
    for _i in 0..=REPEATS {
        let _bobbins: HashSet<u32> = wind(&mut loom.clone(), v3verts, &vert_idx);
    }
    elapsed_ms(start, Instant:: now(), REPEATS, "wind");
}

fn test_reflect_shift () {
    let ba: Vec<[i32; 2]> = vec![[7, 1], [7, -1], [5, -1], [5, -3], [3, -3], [3, -5], [1, -5], [1, -7], [-1, -7], [-1, -5], [-3, -5], [-3, -3], [-5, -3], [-5, -1], [-7, -1], [-7, 1], [-5, 1], [-5, 3], [-3, 3], [-3, 5], [-1, 5], [-1, 7], [1, 7], [1, 5], [3, 5], [3, 3], [5, 3], [5, 1], [3, 1], [3, -1], [1, -1], [1, -3], [-1, -3], [-1, -1], [-3, -1], [-3, 1], [-1, 1], [-1, 3], [1, 3], [1, 1]];
    let a: Array2<i32> =  Array2::from(ba);
    let result: Array2<i32> = reflect(&a);
    let result1: Array2<i32> = shift(result);
    println!("{:?}", result1.len());
    let g32: Adj = adj32::adj32();
    println!("{:?}", g32);
}

fn test_edges() {
    let sequence: &[u32; 32] = &[18, 26, 6, 24, 22, 14, 16, 2, 10, 8, 0, 12, 20, 4, 28, 30, 31, 29, 5, 21, 13, 1, 9, 11, 3, 17, 15, 23, 25, 7, 27, 19];
    let start: Instant = Instant::now();
    for _i in 0..=REPEATS {
        edges2(&sequence);
    }
    elapsed_ms(start, Instant:: now(), REPEATS, "edges2");

    let start: Instant = Instant::now();
    for _i in 0..=REPEATS {
        edges(&sequence);
    }
    elapsed_ms(start, Instant:: now(), REPEATS, "edges");

    assert_eq!(edges(&sequence), edges2(&sequence))
}

fn edges2(data: &[u32; 32]) -> HashSet<(u32, u32)> {
    let mut tojoin = data.clone();
    tojoin.rotate_left(1);
    data.iter()
        .zip(tojoin.iter())
        .map(|(&a, &b)| if a < b {(a, b)} else {(b, a)})
        .collect()
}

fn edges(data: &[u32; 32]) -> HashSet<(u32, u32)> {
    data.iter()
        .zip([&data[1..], &data[..1]].concat().iter())
        .map(|(&a, &b)| if a < b { (a, b) } else { (b, a) })
        .collect()
}
