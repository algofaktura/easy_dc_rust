use super::types::Verti16;

pub fn edist((x, y, z): (i32, i32, i32)) -> i32 {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt().round() as i32
}

pub fn edist16(v: [i16;3]) -> u32 {
    ((v[0].pow(2) + v[1].pow(2) + v[2].pow(2)) as f32).sqrt().round() as u32
}

pub fn edist_i16((x, y, z): Verti16) -> u32 {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt().round() as u32
}

pub fn edist_f32((x, y, z): (i32, i32, i32)) -> f32 {
    ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt()
}

pub fn absumv(v: (i32, i32, i32)) -> u32 {
    (v.0.abs() + v.1.abs() + v.2.abs()) as u32
}

pub fn absumv_i16((x, y, z): Verti16) -> u32 {
    (x.abs() + y.abs() + z.abs()) as u32
}

pub fn absumv1((x, y, z): (i32, i32, i32)) -> i32 {
    x.abs() + y.abs() + z.abs()
}

pub fn get_max_xyz(order: i32) -> i32 {
    (0..order)
        .map(|n| {
            (
                n,
                ((4.0 / 3.0) * (n as f64 + 2.0) * (n as f64 + 1.0) * n as f64).round() as u32,
            )
        })
        .filter(|(_, sum)| *sum == order as u32)
        .map(|(n, _)| n)
        .next()
        .unwrap()
        * 2
        - 1
}

pub fn get_max_xyz_i16(order: i32) -> i16 {
    let result = (0..order)
        .map(|n| {
            (
                n,
                ((4.0 / 3.0) * (n as f64 + 2.0) * (n as f64 + 1.0) * n as f64).round() as u32,
            )
        })
        .filter(|(_, sum)| *sum == order as u32)
        .map(|(n, _)| n)
        .next()
        .unwrap()
        * 2
        - 1;
    result as i16
}

