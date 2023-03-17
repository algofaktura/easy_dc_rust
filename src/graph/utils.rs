pub mod info {
    use super::super::types::{Adjacency, Idx, Point, V2d, V3d, Vert};
    pub fn absumv((x, y, z): Vert) -> Point {
        let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
            let mask = x >> 31;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 31;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn absumv_v3d(v: V3d) -> Point {
        let abs_sum = v.iter().fold(0, |acc, x| {
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

    pub fn get_max_xyz(order: i32) -> Point {
        (0..order)
            .map(|n| (n, get_order_from_n(n as u32)))
            .filter(|(_, sum)| *sum == order as u32)
            .map(|(n, _)| n)
            .next()
            .unwrap()
            * 2
            - 1
    }

    pub fn get_order_from_n(n: u32) -> u32 {
        ((4.0 / 3.0) * ((n + 2) * (n + 1) * n) as f64).round() as u32
    }

    pub fn sum_neighbors(adj: &Adjacency) -> usize {
        adj.values().map(|value| value.len()).sum()
    }

    pub fn is_valid_edge((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
        matches!(
            (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF),
            4..=10
        )
    }
}

pub mod operators {
    use ndarray::{arr2, Array2};

    use crate::graph::types::Point;
    pub fn orient(m: u32, n: u32) -> (u32, u32) {
        if m < n {
            (m, n)
        } else {
            (n, m)
        }
    }

    pub fn shift_xyz(vert: Array2<Point>) -> Array2<Point> {
        vert + arr2(&[
            [2, 0, 0],
            [-2, 0, 0],
            [0, 2, 0],
            [0, -2, 0],
            [0, 0, 2],
            [0, 0, -2],
        ])
    }
}

pub mod iters {
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
}

pub mod two_dimensions {
    use super::super::types::Vert;

    pub fn axis((x, y, _): &Vert, (x1, y1, _): &Vert) -> usize {
        (0..2)
            .find(|&i| [x, y][i] != [x1, y1][i])
            .expect("Something's wrong, the same verts are being compared.")
    }

    pub fn absumv((x, y, _): Vert) -> i32 {
        let abs_sum = [x, y].iter().fold(0, |acc, x| {
            let mask = x >> 31;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 31;
        (abs_sum ^ sign_bit) - sign_bit
    }
}

pub mod version_i16 {

    pub fn abs(n: i16) -> i16 {
        let mask = n >> 15;
        (n + mask) ^ mask
    }

    pub fn absumv((x, y, z): (i16, i16, i16)) -> i16 {
        let abs_sum = [x, y, z].iter().fold(0, |acc, x| {
            let mask = x >> 15;
            acc + (x ^ mask) - mask
        });
        let sign_bit = abs_sum >> 15;
        (abs_sum ^ sign_bit) - sign_bit
    }

    pub fn sum(numbers: &[i16]) -> i16 {
        numbers.iter().fold(0, |acc, &num| {
            let sum = acc ^ num;
            let carry = (acc & num) << 1;
            sum + carry
        })
    }

    pub fn sumbit(numbers: &[i16]) -> i16 {
        numbers
            .iter()
            .fold(0, |acc, num| (acc.wrapping_add(num & 0x7FFF)) & 0x7FFF)
    }
}

pub mod versions {
    use crate::graph::types::Vert;

    pub fn is_valid_edge2((x1, y1, _): Vert, (x2, y2, _): Vert) -> bool {
        matches!(
            (
                (x1 & 0xFFFF) + (y1 & 0xFFFF) + (x2 & 0xFFFF) + (y2 & 0xFFFF),
                (x1 >> 31) + (y1 >> 31) + (x2 >> 31) + (y2 >> 31)
            ),
            (4..=10, 0)
        )
    }
}
