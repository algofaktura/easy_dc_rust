use std::time::{Duration, Instant};

use super::types::Adjacency;

pub fn elapsed_ms(start: Instant, end: Instant, repeats: u32, name: &str) -> f64 {
    let dur: Duration = end - start;
    println!("x{repeats}: {name}() took {} secs", dur.as_secs_f64());
    dur.as_secs_f64()
}

pub fn sum_neighbors(adj: &Adjacency) -> usize {
    adj 
        .values()
        .map(
            |value|
            value.len()
        )
        .sum()
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
