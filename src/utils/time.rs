use std::time::{Instant, Duration};

pub fn elapsed_ms(start: Instant, end: Instant, repeats: u32, name: &str) {
    let dur: Duration = end - start;
    println!("x{repeats}: {name}() took {} secs", dur.as_secs_f64())
}