use std::time::{Instant, Duration};

pub fn elapsed_ms(start: Instant, end: Instant, repeats: u32, name: &str) {
    let dur: Duration = end - start;
    println!("x{repeats}: {name}() took {} secs", dur.as_secs_f64())
}


// fn test_speed() {
//     let start: Instant = Instant::now();
//     for _i in 0..=REPEATS {
//         // do something
//     }
//     elapsed_ms(start, Instant:: now(), REPEATS, "edges");

//     assert_eq!(edges(&sequence), edges2(&sequence))
// }
