use std::time::Instant;

use crate::compare;
use crate::graph::types::Verts;
use crate::utils::time::elapsed_ms;

pub fn is_valid_edge((x1, y1, _): (i32, i32, i32), (x2, y2, _): (i32, i32, i32)) -> bool {
    let total = (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF);
    (4 <= total) && (total <= 10)
}

pub fn is_valid_edge_i16((x1, y1, _): (i16, i16, i16), (x2, y2, _): (i16, i16, i16)) -> bool {
    let total = (x1 as i32 & 0x7FFF) + (y1 as i32 & 0x7FFF) + (x2 as i32 & 0x7FFF) + (y2 as i32 & 0x7FFF);
    (4 <= total) && (total <= 10)
}
    
pub fn is_valid_edge_i16i((x1, y1, _): (i16, i16, i16), (x2, y2, _): (i16, i16, i16)) -> bool {
    let total = (x1 as i32 & 0xFFFF) + (y1 as i32 & 0xFFFF) + (x2 as i32 & 0xFFFF) + (y2 as i32 & 0xFFFF);
    (4 <= total) && (total <= 10)
}

pub fn is_valid_edge21((x1, y1, _): (i32, i32, i32), (x2, y2, _): (i32, i32, i32)) -> bool {
    ((x1 & 0xFFFF + y1 & 0xFFFF + x2 & 0xFFFF + y2 & 0xFFFF) >= 4) && ((x1 & 0xFFFF + y1 & 0xFFFF + x2 & 0xFFFF + y2 & 0xFFFF) <= 10)
}

pub fn is_valid_edge3((x1, y1, _): (i32, i32, i32), (x2, y2, _): (i32, i32, i32)) -> bool {
    match (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF) {
        4..=10 => true,
        _ => false,
    }
}
    
pub fn is_valid_edge2((x1, y1, _): (i32, i32, i32), (x2, y2, _): (i32, i32, i32)) -> bool {
    (4 <= (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF)) &&
    (10 >= (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF))
}

pub fn test_is_valid_edge(order: u32, _max_xyz: i32, verts: Verts, _repeats: u32) {
    extern crate rand;
    use rand::Rng;
    use compare::is_valid_edge;

    let mut rng = rand::thread_rng();

    let left: (i32, i32, i32) = verts[rng.gen_range(0..order) as usize];
    let mut a: bool = true;
    let mut b: bool = true;
    // let mut c: bool = true;

    let start: Instant = Instant::now();
    for vert in &verts {
        for _ in 0..1000 {
            a = is_valid_edge::is_valid_edge(left, *vert);
        }
    }
    let dur = elapsed_ms(start, Instant::now(), order, "is_valid_edge");    
    println!("A {:?} | DUR: {:?}", a, dur);


    let start: Instant = Instant::now();
    for vert in &verts {
        
        if is_valid_edge::is_valid_edge(left, *vert) == is_valid_edge::is_valid_edge2(left, *vert) {

        } else {
            println!("{:?} {:?}", left, vert)
        }
        
    }
    let dur = elapsed_ms(start, Instant::now(), order, "compare");    
    println!("x: {} | DUR: {:?}", b, dur);


    let start: Instant = Instant::now();
    for vert in &verts {
        for _ in 0..1000 {
            b = is_valid_edge::is_valid_edge2(left, *vert);
        }
    }
    let dur = elapsed_ms(start, Instant::now(), order, "is_valid_edge2");    
    println!("2: {} | DUR: {:?}", b, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        for _ in 0..1000 {
            a = is_valid_edge::is_valid_edge3(verts[rng.gen_range(0..order) as usize], *vert);
        }
    }
    let dur = elapsed_ms(start, Instant::now(), order, "is_valid_edge3");    
    println!("3 {:?} | DUR: {:?}", a, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        for _ in 0..1000 {
            a = is_valid_edge::is_valid_edge2(verts[rng.gen_range(0..order) as usize], *vert);
        }
    }
    let dur = elapsed_ms(start, Instant::now(), order, "is_valid_edge2");    
    println!("A {:?} | DUR: {:?}", a, dur);

}
