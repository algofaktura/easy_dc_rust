use ndarray::{Axis, Slice};

use super::spool;
use crate::graph::types::{
    Adjacency, Bobbins, Count, Loom, Point, Spool, Thread, Tour, VIMap, Varr, Verts, Warps,
    Weights, Woven, Yarn, ZOrder,
};

pub fn warp_loom(
    vert_idx: &VIMap,
    verts: &Verts,
    var: &Varr,
    weights: &Weights,
    z_adj: &Adjacency,
    z_length: &ZOrder,
) -> Loom {
    let spool: Spool = spool::yarn(&z_adj, verts, var, weights);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_length {
        let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vert_idx);
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = spool::wind(&mut loom, verts, &vert_idx);
        }
    }
    reflect_solution(&mut loom, verts, vert_idx);
    loom.sort_by_key(|w| w.len());
    loom
}

pub fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vert_idx: &VIMap,
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
        spool::cut(node_yarn, &bobbins)
    }
}

pub fn get_node_yarn(mut yarn: Yarn, zlevel: Point, order: Count, vert_idx: &VIMap) -> Tour {
    yarn.slice_axis_inplace(
        Axis(0),
        Slice::new((yarn.len_of(Axis(0)) - order).try_into().unwrap(), None, 1),
    );
    yarn.outer_iter()
        .map(|row| vert_idx[&(row[0], row[1], zlevel)])
        .collect()
}

pub fn join_threads(loom: &mut Loom, warps: &Warps) -> Woven {
    let mut woven: Woven = Woven::new();
    for thread in loom {
        for (idx, warp) in warps.iter().enumerate() {
            if !woven.contains(&idx) {
                match (thread.front(), thread.back()) {
                    (Some(front), _) if *front == warp[0] => {
                        *thread = warp[1..]
                            .iter()
                            .rev()
                            .chain(thread.iter())
                            .cloned()
                            .collect()
                    }
                    (_, Some(back)) if *back == warp[0] => thread.extend(warp.iter().skip(1)),
                    _ => continue,
                }
                woven.extend([idx])
            }
        }
    }
    woven
}

// Affix loose threads to loom. Threads that have not been incorporated.
pub fn affix_loose_threads(loom: &mut Loom, warps: Warps, woven: Woven) {
    for (_, seq) in warps
        .iter()
        .enumerate()
        .filter(|(idx, _)| !woven.contains(idx))
    {
        loom.extend(vec![Thread::from(seq.iter().cloned().collect::<Thread>())])
    }
}

pub fn reflect_solution(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|(x, y, z)| vert_idx[&(x, y, -z)])
                .collect::<Tour>(),
        )
    }
}
