pub fn uon(start: usize, end: usize, max_n: usize) -> impl Iterator<Item = usize> {
    (0..max_n + 2).map(move |i| {
        let _uon = (0..max_n * 2 + 2).step_by(2)
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
