/////////////////////////////////////////////////////////////////////////////
/// N: ORDER
/// 1: 8
/// 2: 32
/// 3: 80
/// 4: 160
/// 5: 280
/// 6: 448
/// 7: 672
/// 8: 960
/// 9: 1320
/// 10: 1760
/// 11: 2288
/// 12: 2912
/// 13: 3640
/// 14: 4480
/// 15: 5440
/// 16: 6528
/// 17: 7752
/// 18: 9120
/// 19: 10640
/// 20: 12320
/// 21: 14168
/// 22: 16192
/// 23: 18400
/// 24: 20800
/// 25: 23400
/// 26: 26208
/// 27: 29232
/// 28: 32480
/// 29: 35960
/// 30: 39680
/// 31: 43648
/// 32: 47872
/// 33: 52360
/// 34: 57120
/// 35: 62160
/// 36: 67488
/// 37: 73112
/// 38: 79040
/// 39: 85280
/// 40: 91840
/// 41: 98728
/// 42: 105952
/// 43: 113520
/// 44: 121440
/// 45: 129720
/// 46: 138368
/// 47: 147392
/// 48: 156800
/// 49: 166600
/// 50: 176800
/// 51: 187408
/// 52: 198432
/// 53: 209880
/// 54: 221760
/// 55: 234080
/// 56: 246848
/// 57: 260072
/// 58: 273760
/// 59: 287920
/// 60: 302560
/// 61: 317688
/// 62: 333312
/// 63: 349440
/// 64: 366080
/// 65: 383240
/// 66: 400928
/// 67: 419152
/// 68: 437920
/// 69: 457240
/// 70: 477120
/// 71: 497568
/// 72: 518592
/// 73: 540200
/// 74: 562400
/// 75: 585200
/// 76: 608608
/// 77: 632632
/// 78: 657280
/// 79: 682560
/// 80: 708480
/// 81: 735048
/// 82: 762272
/// 83: 790160
/// 84: 818720
/// 85: 847960
/// 86: 877888
/// 87: 908512
/// 88: 939840
/// 89: 971880
/// 90: 1004640
/// 91: 1038128
/// 92: 1072352
/// 93: 1107320
/// 94: 1143040
/// 95: 1179520
/// 96: 1216768
/// 97: 1254792
/// 98: 1293600
/// 99: 1333200
/// 100: 1373600
/////////////////////////////////////////////////////////////////////////////
extern crate rayon;

use std::{
    env,
    time::Instant, f32::INFINITY,
};

pub mod graph;

use graph::{
    check, 
    types::*, 
    weave
};

/// see n_order.txt for a list of n and the corresponding order:
/// cargo run --release [N] [N_UPPER_INCLUSIVE][REPEATS]
pub fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args
        .get(1)
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);
    let n_upper: u32 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);
    let repeats: u32 = args
        .get(3)
        .unwrap_or(&"1".to_string())
        .parse()
        .unwrap_or(1);
    println!("");
    for level in n..=n_upper {
        if (level - 1) % 75 == 0 {
            println!("");
            println!("");
        }
        find_solution(graph::make::make_graph(level), repeats)
    }
}

pub fn find_solution(
    (n, order, verts, vi_map, adj, edge_adj, z_adj, z_order): (
        u32,
        u32,
        Verts,
        VIMap,
        Adjacency,
        EdgeAdjacency,
        Adjacency,
        ZOrder,
    ),
    repeats: u32,
) {
    let mut min_dur = INFINITY;
    let mut solution: Solution = Solution::new();
    for _ in 0..repeats {
        let start: Instant = Instant::now();
        solution = weave::weave(&adj, &vi_map, &edge_adj, &verts, &z_adj, &z_order);
        let dur = (Instant::now() - start).as_secs_f32();
        if min_dur > dur {
            min_dur = dur
        }

    }
    let seq_id = check::id_seq(&solution, &adj);
    println!(
        "| ???? {:>4} | ?????? {:>10} | ???? {:>14.7} | ???? {:?} |",
        n,  
        order,
        min_dur, 
        seq_id,
    );
}
