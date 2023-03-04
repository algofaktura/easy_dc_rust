use ndarray::{Axis, Slice};

use crate::{types::types::{Adjacency, Bobbins, Loom, Path, Spool, Thread, Vectors3d, VertIdx, VertsC3, Warps, Woven, Yarn}, graphs::utils::{shrink::shrink_adjacency, map::vectorize}, structs::vector::Vector3D};
use super::{spool::spool_yarn, cut::cut, wind::wind};

pub fn warp_loom(v3verts: &Vectors3d, adj: &Adjacency, vert_idx: &VertIdx, verts: &VertsC3, var: &[[i32; 3]]) -> Loom {
    let (z_adj, z_length) = shrink_adjacency(&v3verts, &adj);
    let spool: Spool = spool_yarn(&z_adj, verts, var);
    let mut bobbins: Bobbins = Vec::new();
    let mut warps: Warps;
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_length {
        let mut yarn: Yarn = spool[&(zlevel % 4 + 4).try_into().unwrap()].clone();
        yarn.slice_axis_inplace(Axis(0), Slice::new((yarn.len_of(Axis(0)) - order).try_into().unwrap(), None, 1));
        let node_yarn: Path = yarn.outer_iter().map(|row| Vector3D::to_node(row[0], row[1], zlevel, &vert_idx)).collect();
        if bobbins.is_empty() { warps = vec![node_yarn] } else { warps = cut(node_yarn, &bobbins) }
        let mut woven: Woven = Woven::new();
        for thread in &mut loom {
            for (idx, warp) in warps.iter().enumerate() {
                if !woven.contains(&idx) {
                    for end in vec![0 as usize, thread.len() - 1] {
                        if thread[end] == warp[0 as usize] {
                            woven.extend([idx]);
                            if end == 0 as usize { 
                                warp[1..].iter().map(|item| thread.push_front(*item)).fold((), |_, _| ());
                            } else { 
                                thread.extend(&warp[1..]) 
                            }
                        }
                    }
                }
            }
        }
        for (_, seq) in warps.iter().enumerate().filter(|(idx, _)| !woven.contains(idx)) {
            loom.extend(vec![Thread::from(seq.iter().cloned().collect::<Thread>())]);
        }
        if zlevel == -1 { break }
        bobbins = wind(&mut loom, &vectorize(verts), &vert_idx);
    }
    for thread in &mut loom {
        let nodes: Path = thread.iter().map(|&node| v3verts[node as usize].mirror_z(&vert_idx)).collect();
        thread.extend(nodes.into_iter().rev());
    }
    loom.sort_by_key(|w| w.len());
    loom
}
