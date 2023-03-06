use ndarray::{Axis, Slice};

use super::{cut::cut, spool::spool_yarn, wind::wind};
use crate::{
    graphs::utils::{map::vectorize, shrink::shrink_adjacency},
    structs::vector::Vector3D,
    types::types::{
        Adjacency, Bobbins, Count, Loom, Point, Spool, Thread, Tour, Vectors3d, VertIdx, VertsC3,
        Warps, Woven, Yarn,
    },
};

pub fn warp_loom(
    v3verts: &Vectors3d,
    adj: &Adjacency,
    vert_idx: &VertIdx,
    verts: &VertsC3,
    var: &[[i32; 3]],
) -> Loom {
    let (z_adj, z_length) = shrink_adjacency(&v3verts, &adj);
    let spool: Spool = spool_yarn(&z_adj, verts, var);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_length {
        let warps: Warps = get_warps(zlevel, order, &bobbins, &spool, vert_idx);
        let woven: Woven = join_threads(&mut loom, &warps);
        add_leftovers_to_loom(&mut loom, warps, woven);
        if zlevel != -1 {
            bobbins = wind(&mut loom, &vectorize(verts), &vert_idx);
        }
    }
    reflect_solution(&mut loom, v3verts, vert_idx);
    loom.sort_by_key(|w| w.len());
    loom
}

pub fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vert_idx: &VertIdx,
) -> Warps {
    let node_yarn: Tour = get_node_yarn(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vert_idx,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut(node_yarn, &bobbins)
    }
}

pub fn get_node_yarn(mut yarn: Yarn, zlevel: Point, order: Count, vert_idx: &VertIdx) -> Tour {
    yarn.slice_axis_inplace(
        Axis(0),
        Slice::new((yarn.len_of(Axis(0)) - order).try_into().unwrap(), None, 1),
    );
    yarn.outer_iter()
        .map(|row| Vector3D::to_node(row[0], row[1], zlevel, &vert_idx))
        .collect()
}

pub fn join_threads(loom: &mut Loom, warps: &Warps) -> Woven {
    let mut woven: Woven = Woven::new();
    for thread in loom {
        for (idx, warp) in warps.iter().enumerate() {
            if !woven.contains(&idx) {
                match (thread.front(), thread.back()) {
                    (Some(f), _) if *f == warp[0] => {
                        *thread = warp[1..]
                            .iter()
                            .rev()
                            .chain(thread.iter())
                            .cloned()
                            .collect()
                    }
                    (_, Some(b)) if *b == warp[0] => thread.extend(warp.iter().skip(1)),
                    _ => {
                        continue;
                    }
                }
                woven.extend([idx])
            }
        }
    }
    woven
}

pub fn add_leftovers_to_loom(loom: &mut Loom, warps: Warps, woven: Woven) {
    for (_, seq) in warps
        .iter()
        .enumerate()
        .filter(|(idx, _)| !woven.contains(idx))
    {
        loom.extend(vec![Thread::from(seq.iter().cloned().collect::<Thread>())])
    }
}

pub fn reflect_solution(loom: &mut Loom, v3verts: &Vectors3d, vert_idx: &VertIdx) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| v3verts[node as usize].mirror_z(vert_idx))
                .collect::<Tour>(),
        )
    }
}
