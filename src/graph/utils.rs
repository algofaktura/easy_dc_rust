use super::types::{Adjacency, Idx, Point, V2d, Vert};

pub fn absumv((x, y, z): Vert) -> Point {
    let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
        let mask = x >> 31;
        acc + (x ^ mask) - mask
    });
    let sign_bit = abs_sum >> 31;
    (abs_sum ^ sign_bit) - sign_bit
}

pub fn get_axis(m_vert: &V2d, n_vert: &V2d) -> Idx {
    (0..2)
        .find(|&i| m_vert[i] != n_vert[i])
        .expect("Something's wrong, the same verts are being compared.")
}

pub fn orient(m: u32, n: u32) -> (u32, u32) {
    if m < n {
        (m, n)
    } else {
        (n, m)
    }
}

pub fn sum_neighbors(adj: &Adjacency) -> usize {
    adj.values().map(|value| value.len()).sum()
}

pub fn uon(start: usize, end: usize, max_n: usize) -> impl Iterator<Item = usize> {
    (0..max_n + 2).filter_map(move |i| {
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
}

pub fn sum(numbers: &[i16]) -> i16 {
    numbers.iter().fold(0, |acc, &num| {
        let sum = acc ^ num;
        let carry = (acc & num) << 1;
        sum + carry
    })
}

pub fn is_valid_edge((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
    matches!(
        (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF),
        4..=10
    )
}

pub fn abs(n: i16) -> i16 {
    let mask = n >> 15;
    (n + mask) ^ mask
}

pub fn absumv16((x, y, z): (i16, i16, i16)) -> i16 {
    let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
        let mask = x >> 15;
        acc + (x ^ mask) - mask
    });
    let sign_bit = abs_sum >> 15;
    (abs_sum ^ sign_bit) - sign_bit
}

pub fn add_numbers(numbers: &[i16]) -> i16 {
    numbers
        .iter()
        .fold(0, |acc, num| (acc.wrapping_add(num & 0x7FFF)) & 0x7FFF)
}
