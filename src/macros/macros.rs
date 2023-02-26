#[allow(unused_macros)]
macro_rules! set {
    ($($s:expr),*) => {HashSet::from([$($s),*])}
}