use std::time::Instant;

use crate::graph::make::vertices::make_vertices;
use crate::graph::measure::get_max_xyz;
use crate::graph::types::Verts;
use crate::utils::time::elapsed_ms;

pub fn absumv((x, y, z): (i32, i32, i32)) -> i32 {
    [x, y, z].iter().map(|&n| ((n >> 31) ^ n).wrapping_sub(n >> 31)).sum()
}

pub fn absumv1((x, y, z): (i32, i32, i32)) -> u32 {
    [x, y, z].iter().map(|&n| n.abs() as u32).fold(0, |sum, n| sum + n)
}

pub fn absumv2((x, y, z): (i32, i32, i32)) -> i32 {
    ((x ^ x >> 31) - (x >> 31)) + ((y ^ y >> 31) - (y >> 31)) + ((z ^ z >> 31) - (z >> 31))
}

pub fn absumv3((x, y, z): (i32, i32, i32)) -> u32 {
    let mask = (x >> 31) | (y >> 31) | (z >> 31);
    let x_abs = (x ^ mask) - mask;
    let y_abs = (y ^ mask) - mask;
    let z_abs = (z ^ mask) - mask;
    (x_abs + y_abs + z_abs) as u32
}

pub fn absumv4((x, y, z): (i32, i32, i32)) -> u32 {
    ((x ^ (x.signum() * -1)).wrapping_sub(x.signum() * -1) as u32)
    + ((y ^ (y.signum() * -1)).wrapping_sub(y.signum() * -1) as u32)
    + ((z ^ (z.signum() * -1)).wrapping_sub(z.signum() * -1) as u32)
}

pub fn absumv5((x, y, z): (i32, i32, i32)) -> u32 {
    (((x >> 31) ^ x).wrapping_sub(x >> 31) as u32)
    + (((y >> 31) ^ y).wrapping_sub(y >> 31) as u32)
    + (((z >> 31) ^ z).wrapping_sub(z >> 31) as u32)
}

pub fn absumv6((x, y, z): (i32, i32, i32)) -> u32 {
    (x.abs() + y.abs() + z.abs()) as u32
}


pub fn test_absumv(order: u32) {
    let max_xyz = get_max_xyz(order as i32);
    let verts: Verts = make_vertices(max_xyz);

    let mut a: i32 = 0;
    let mut b: u32 = 0;
    let mut c: i32 = 0;
    let mut d: u32 = 0;
    let mut e: u32 = 0;
    let mut f: u32 = 0;
    let mut g: u32 = 0;

    let start: Instant = Instant::now();
    for vert in &verts {
        a = absumv(*vert);
    }
    let dur = elapsed_ms(start, Instant::now(), order, "absumv");    
    println!("A {:?} | DUR: {:?}", a, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        b = absumv1(*vert);
    }
    let dur = elapsed_ms(start, Instant::now(), order, "absumv1");    
    println!("B: {} | DUR: {:?}", b, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        c = absumv2(*vert);
    }
    let dur = elapsed_ms(start, Instant::now(), order, "absumv2");    
    println!("C: {:?} | DUR: {:?}", c, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        d = absumv3(*vert);
    }
    let dur = elapsed_ms(start, Instant::now(), order, "absumv3");    
    println!("D: {:?} | DUR: {:?}", d, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        e = absumv4(*vert);
    }
    let dur = elapsed_ms(start, Instant::now(), order, "absumv4");    
    println!("E: {:?} | DUR: {:?}", e, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        f = absumv5(*vert);
    }
    let dur = elapsed_ms(start, Instant::now(), order, "absumv5");    
    println!("F: {:?} | DUR: {:?}", f, dur);

    let start: Instant = Instant::now();
    for vert in &verts {
        g = absumv6(*vert);
    }
    let dur = elapsed_ms(start, Instant::now(), order, "absumv6");    
    println!("G: {:?} | DUR: {:?}", g, dur);

}