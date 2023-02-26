pub fn flip(ix: usize, path: &mut [i32], end: bool) {
    if end {
        path[ix + 1..].reverse();
    } else {
        path[..ix].reverse();
    }
}