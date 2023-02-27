use common_macros::hash_map;

use crate::types::types::Adj;
use crate::enums::enums::Neighbors;

pub fn adj32() -> Adj {
    let graph = hash_map! {
        0 => Neighbors::Six([8, 12, 14, 1, 2, 4]),
        1 => Neighbors::Six([9, 13, 15, 0, 3, 5]),
        2 => Neighbors::Six([10, 16, 18, 0, 3, 6]),
        3 => Neighbors::Six([11, 17, 19, 1, 2, 7]),
        4 => Neighbors::Six([20, 22, 28, 0, 5, 6]),
        5 => Neighbors::Six([21, 23, 29, 1, 4, 7]),
        6 => Neighbors::Six([24, 26, 30, 2, 4, 7]),
        7 => Neighbors::Six([25, 27, 31, 3, 5, 6]),
        8 => Neighbors::Three([9, 10, 0]),
        9 => Neighbors::Three([8, 11, 1]),
        10 => Neighbors::Three([8, 11, 2]),
        11 => Neighbors::Three([9, 10, 3]),
        12 => Neighbors::Three([20, 13, 0]),
        13 => Neighbors::Three([12, 21, 1]),
        14 => Neighbors::Three([16, 22, 0]),
        15 => Neighbors::Three([17, 23, 1]),
        16 => Neighbors::Three([24, 14, 2]),
        17 => Neighbors::Three([25, 15, 3]),
        18 => Neighbors::Three([26, 19, 2]),
        19 => Neighbors::Three([18, 27, 3]),
        20 => Neighbors::Three([12, 21, 4]),
        21 => Neighbors::Three([20, 13, 5]),
        22 => Neighbors::Three([24, 14, 4]),
        23 => Neighbors::Three([25, 15, 5]),
        24 => Neighbors::Three([16, 22, 6]),
        25 => Neighbors::Three([17, 23, 7]),
        26 => Neighbors::Three([18, 27, 6]),
        27 => Neighbors::Three([26, 19, 7]),
        28 => Neighbors::Three([29, 30, 4]),
        29 => Neighbors::Three([28, 31, 5]),
        30 => Neighbors::Three([28, 31, 6]),
        31 => Neighbors::Three([29, 30, 7]),
    };
    graph
}