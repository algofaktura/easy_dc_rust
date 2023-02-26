use common_macros::hash_map;
use common_macros::hash_set;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let hashed: HashMap<u32, HashSet<u32>> = hash_map!{
        1 => hash_set![1, 2, 3],
        2 => hash_set![3, 4, 5],
    };
    println!("{:?}", hashed)
}