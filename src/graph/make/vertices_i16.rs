use itertools::Itertools;

use crate::graph::measure::{absumv, absumv16, edist, edist16};

pub fn make_vertices16(max_xyz: i16) -> Vec<[i16;3]> {
    (-(max_xyz)..=(max_xyz))
        .step_by(2)
        .flat_map(|x| {
            (-(max_xyz as i16)..=(max_xyz as i16))
                .step_by(2)
                .flat_map(move |y| {
                    (-(max_xyz as i16)..=(max_xyz as i16))
                        .step_by(2)
                        .map(move |z| [x, y, z])
                        .filter(|&v| absumv16(v) < (max_xyz + 4) as u32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .sorted_by_key(|v| (edist16(*v), v[0], v[1], v[2]))
        .collect::<Vec<[i16;3]>>()
}
