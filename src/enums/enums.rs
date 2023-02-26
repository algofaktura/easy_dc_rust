trait Degree {
    type Three;
    type Six;
}

struct Neighbors;

impl Degree for Neighbors {
    type Three = [i32; 3];
    type Six = [i32; 6];
}

pub fn tryout() {
    let degree3: <Neighbors as Degree>::Three = [1, 2, 3];
    let degree6: <Neighbors as Degree>::Six = [1, 2, 3, 1, 2, 3];
    println!("{:?} {:?}", degree3, degree6)
}