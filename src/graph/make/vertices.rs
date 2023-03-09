use itertools::Itertools;

use crate::graph::measure::{absumv, absumv_i16, edist, edist_i16};
use crate::graph::types::{Verts, Verti16};

pub fn make_vertices(max_xyz: i32) -> Verts {
    (-(max_xyz)..=(max_xyz))
        .step_by(2)
        .flat_map(|x| {
            (-(max_xyz as i32)..=(max_xyz as i32))
                .step_by(2)
                .flat_map(move |y| {
                    (-(max_xyz as i32)..=(max_xyz as i32))
                        .step_by(2)
                        .map(move |z| (x, y, z))
                        .filter(|&v| absumv(v) < (max_xyz + 4) as u32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .sorted_by_key(|v| (edist(*v), v.0, v.1, v.2))
        .collect::<Vec<_>>()
}

pub fn make_vertices_i16(max_xyz: i16) -> Vec<Verti16> {
    (-(max_xyz)..=(max_xyz))
        .step_by(2)
        .flat_map(|x| {
            (-(max_xyz as i16)..=(max_xyz as i16))
                .step_by(2)
                .flat_map(move |y| {
                    (-(max_xyz as i16)..=(max_xyz as i16))
                        .step_by(2)
                        .map(move |z| (x, y, z))
                        // .map(move |z| [x, y, z])
                        .filter(|&v| absumv_i16(v) < (max_xyz + 4) as u32)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .sorted_by_key(|v| (edist_i16(*v), v.0, v.1, v.2))
        .collect::<Vec<Verti16>>()
}
