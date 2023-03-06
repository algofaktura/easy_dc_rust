use std::time::{Duration, Instant};

pub fn elapsed_ms(start: Instant, end: Instant, repeats: u32, name: &str) {
    let dur: Duration = end - start;
    println!("x{repeats}: {name}() took {} secs", dur.as_secs_f64())
}

// struct T;

// fn speed_testT<T>(repeats: u32, data: T, f: & dyn Fn(T) -> T) -> T {
//     let mut _result: Option<T> = None;
//     let start = Instant::now();
//     for _ in 0..repeats {
//         _result = Some(f(&data));
//     }
//     elapsed_ms(start, Instant::now(), repeats, "WEAVE");
//     _result.expect("No result found")
// }

// fn speed_test<T>(repeats: u32) {
//     let mut _result: Option<T> = None;
//     let start = Instant::now();
//     for _ in 0..repeats {
//         _result = Some();
//     }
//     elapsed_ms(start, Instant::now(), repeats, "WEAVE");
//     _result.expect("No result found")
// }
