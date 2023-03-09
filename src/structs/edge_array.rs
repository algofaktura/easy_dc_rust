use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Frozen {
    data: [u32;2]
}

impl<'a> Frozen<'a> {
    pub fn new(lhs: &'a (i32, i32, i32), rhs: &'a (i32, i32, i32)) -> Frozen<'a> {
        if lhs <= rhs {
            Frozen(lhs, rhs)
        } else {
            Frozen(rhs, lhs)
        }
    }

    pub fn lhs(&self) -> &'a (i32, i32, i32) {
        self.0
    }

    pub fn rhs(&self) -> &'a (i32, i32, i32) {
        self.1
    }
    
    pub fn is_valid_edge(&self) -> bool {

        let total = (self.0.0 & 0xFFFF) + (self.0.1 & 0xFFFF) + (self.1.0 & 0xFFFF) + (self.1.1 & 0xFFFF);
        (4 <= total) && (total <= 10)
    }

    pub fn get_axis(&self) -> Idx {
        // not optimal would like to use indexing (0..2)
        match (self.0 .0 != self.1 .0, self.0 .1 != self.1 .1, self.0 .2 != self.1 .2) {
            (true, false, false) => 0,
            (false, true, false) => 1,
            (false, false, true) => 2,
            _ => panic!("VERTS ARE SIMILAR"),
        }
    }
}

impl<'a> PartialEq for Frozen<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<'a> Eq for Frozen<'a> {}

impl<'a> Hash for Frozen<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();
        let mut tuple = (self.0, self.1);
        tuple.0 = &minmax(tuple.0, tuple.1);
        tuple.1 = &maxmin(tuple.0, tuple.1);
        tuple.hash(&mut hasher);
        hasher.finish().hash(state);
    }
}

impl<'a> Ord for Frozen<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self
            .iter()
            .map(|&(x, y, z)| x.abs() + y.abs() + z.abs())
            .sum()
            .cmp(
                &other.iter().map(|&(x, y, z)| x.abs() + y.abs() + z.abs()).sum()
            )
    }
}


impl<'a> PartialOrd for Frozen<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> std::fmt::Display for Frozen<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lhs = minmax(self.0, self.1);
        let rhs = maxmin(self.0, self.1);
        write!(f, "({}, {})", lhs.0, lhs.1)
    }
}

fn minmax<'a>((a, b, _): &'a (i32, i32, i32), (d, e, _): &'a (i32, i32, i32)) -> &'a (i32, i32) {
    if (a, b) < (d, e) {
        &(a, b)
    } else {
        &(d, e)
    }
}

fn maxmin<'a>((a, b, _): &'a (i32, i32, i32), (d, e, _): &'a (i32, i32, i32)) -> &'a (i32, i32) {
    if (a, b) > (d, e) {
        &(a, b)
    } else {
        &(d, e)
    }
}
