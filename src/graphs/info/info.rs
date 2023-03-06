pub fn is_valid_edge((x1, y1, _): (i32, i32, i32), (x2, y2, _): (i32, i32, i32)) -> bool {
    let total = (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF);
    (4 <= total) && (total <= 10)
}
