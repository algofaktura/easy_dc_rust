use std::time::{Duration, Instant};

use super::types::{Adjacency, Idx, Point, V2d};

pub fn elapsed_ms(start: Instant, end: Instant, repeats: u32, name: &str) -> f64 {
    let dur: Duration = end - start;
    println!("x{repeats}: {name}() took {} secs", dur.as_secs_f64());
    dur.as_secs_f64()
}

pub fn sum_neighbors(adj: &Adjacency) -> usize {
    adj.values().map(|value| value.len()).sum()
}

pub fn uon(start: usize, end: usize, max_n: usize) -> impl Iterator<Item = usize> {
    (0..max_n + 2)
        .map(move |i| {
            let _uon = (0..max_n * 2 + 2)
                .step_by(2)
                .take(i)
                .map(|n| n * (n + 2))
                .sum();
            if _uon >= start && _uon <= end {
                Some(_uon)
            } else {
                None
            }
        })
        .flatten()
}

pub fn orient<T: std::cmp::PartialOrd>(m: T, n: T) -> (T, T) {
    if m < n {
        (m, n)
    } else {
        (n, m)
    }
}

pub fn get_axis(m_vert: &V2d, n_vert: &V2d) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("Something's wrong, the same verts are being compared.")
}


pub fn absumv(v: &[Point]) -> Point {
    let abs_sum = v.iter().fold(0, |acc, x| {
        let mask = x >> 31;
        acc + (x ^ mask) - mask
    });
    let sign_bit = abs_sum >> 31;
    (abs_sum ^ sign_bit) - sign_bit
}
