#[derive(Debug)]
pub enum Neighbors {
    Three([u32; 3]),
    Six([u32; 6])
}

trait Degree {
    type Three;
    type Six;
}

struct Neighbor;

impl Degree for Neighbor {
    type Three = [i32; 3];
    type Six = [i32; 6];
}

pub fn tryout() {
    let degree3: <Neighbor as Degree>::Three = [1, 2, 3];
    let degree6: <Neighbor as Degree>::Six = [1, 2, 3, 1, 2, 3];
    println!("{:?} {:?}", degree3, degree6)
}
