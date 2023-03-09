use std::collections::{VecDeque, HashMap};

use ndarray::{Axis, Slice};

use super::spool_ref;
use crate::{
    graph::shrink_ref::shrink_adjacency_ref,
    graph::types::{
        AdjacencyRef, Count, Spool, VertIdxRef, Verts,
        Woven, Yarn, LoomRef, TourRef, ThreadRef, BobbinsRef, WovenRef,
    },
};

pub fn prepare_loom<'a>(
    adj: &'a AdjacencyRef,
    vert_idx: &'a VertIdxRef,
    verts: &'a Verts,
) -> LoomRef<'a> {
    let (z_adj, z_length) = shrink_adjacency_ref(&verts, &adj);
    let spool: Spool = spool_ref::yarn(&z_adj, vert_idx);
    let mut bobbins: BobbinsRef = Vec::new();
    let mut loom: LoomRef = LoomRef::new();
    for (zlevel, order) in z_length {
        let node_yarn: TourRef = get_node_yarn(spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(), order, vert_idx);
        let warps = {if bobbins.is_empty() {
            vec![node_yarn]
        } else {
            spool_ref::cut(node_yarn, bobbins.clone())
        }};
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if zlevel != -1 {
            (bobbins, loom) = spool_ref::wind(loom, &vert_idx);
        }
    }
    reflect_solution(&mut loom, vert_idx);
    loom.sort_by_key(|w| w.len());
    loom
}

pub fn get_node_yarn<'a>(mut yarn: Yarn, order: Count, vert_idx: &'a VertIdxRef) -> TourRef<'a> {
    yarn.slice_axis_inplace(
        Axis(0),
        Slice::new((yarn.len_of(Axis(0)) - order).try_into().unwrap(), None, 1),
    );
    yarn.outer_iter()
        .map(|v| *vert_idx.get(&(v[0], v[1], v[2])).unwrap())
        .collect()
}

pub fn join_threads<'a, 'c>(loom: &'a mut Vec<VecDeque<&'c (i32, i32, i32)>>, warps: &'a Vec<Vec<&'c (i32, i32, i32)>>) -> WovenRef<'a> {
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

pub fn affix_loose_threads<'a, 'b, 'c>(loom: &'a mut Vec<VecDeque<&'c (i32, i32, i32)>>, warps: Vec<Vec<&'c (i32, i32, i32)>>, woven: Woven) {
    for (_, seq) in warps
        .iter()
        .enumerate()
        .filter(|(idx, _)| !woven.contains(idx))
    {
        loom.extend(vec![ThreadRef::from(seq.iter().cloned().collect::<ThreadRef>())])
    }
}

pub fn reflect_solution<'a, 'c>(loom: &'a mut Vec<VecDeque<&'c (i32, i32, i32)>>, vert_idx: &HashMap<(i32, i32, i32), &'c (i32, i32, i32)>) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| *vert_idx.get(&(node.0, node.1, -node.2)).unwrap())
                .collect::<TourRef>(),
        )
    }
}
