use common_macros::hash_map;

use crate::types::types::Adj;
use crate::enums::enums::Neighbors;

pub fn adj8() -> Adj {
    let graph = hash_map! {
        0 => Neighbors::Three([1, 2, 4]),
        1 => Neighbors::Three([0, 3, 5]),
        2 => Neighbors::Three([0, 3, 6]),
        3 => Neighbors::Three([1, 2, 7]),
        4 => Neighbors::Three([0, 5, 6]),
        5 => Neighbors::Three([1, 4, 7]),
        6 => Neighbors::Three([2, 4, 7]),
        7 => Neighbors::Three([3, 5, 6]),
    };
    graph
}