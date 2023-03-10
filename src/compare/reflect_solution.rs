
pub fn reflect_solution(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|(x, y, z)| vert_idx[&(x, y, -z)])
                .collect::<Tour>()
        )
    }
}

pub fn reflect_solutionvec(loom: &mut Loom, v3verts: &Vectors3d, vert_idx: &VIMap) {
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

pub fn reflect_solution2(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| mirror_z(verts[node as usize], vert_idx))
                .collect::<Tour>(),
        )
    }
}

pub fn mirror_z((x, y, z): (Point, Point, Point), vert_idx: &VIMap) -> Node {
    *vert_idx
        .get(&(x, y, -z))
        .unwrap()
}

pub fn reflect_solution3(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| mirror_z(verts[node as usize], vert_idx))
                .collect::<Tour>()
        )
    }
}

pub fn reflect_solution56(loom: &mut Loom, verts: &Verts, vert_idx: &VIMap) {
    for thread in loom {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| vert_idx[&{  
                    let (x, y, z) = verts[node as usize];
                    (x, y, -z)}]
                )
                .collect::<Tour>()
        )
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
                .collect::<Tour>()
        )
    }
}
