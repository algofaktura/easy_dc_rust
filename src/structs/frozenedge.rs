use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Frozen<'a>(&'a (i32, i32, i32), &'a (i32, i32, i32));

impl<'a> Frozen<'a> {
    pub fn new((lhs, rhs): (&'a (i32, i32, i32), &'a (i32, i32, i32))) -> Frozen<'a> {
        if lhs <= rhs {
            Frozen(lhs, rhs)
        } else {
            Frozen(rhs, lhs)
        }
    }

    pub fn left(&self) -> &'a (i32, i32, i32) {
        self.0
    }

    pub fn right(&self) -> &'a (i32, i32, i32) {
        self.1
    }
    
    pub fn is_valid_edge(&self) -> bool {
        let total = (self.0.0 & 0xFFFF) + (self.0.1 & 0xFFFF) + (self.1.0 & 0xFFFF) + (self.1.1 & 0xFFFF);
        (4 <= total) && (total <= 10)
    }

    pub fn get_axis(&self) -> usize {
        // not optimal would like to use indexing (0..2)
        match (self.0 .0 != self.1 .0, self.0 .1 != self.1 .1, self.0 .2 != self.1 .2) {
            (true, false, false) => 0,
            (false, true, false) => 1,
            (false, false, true) => 2,
            _ => panic!("VERTS ARE SIMILAR"),
        }
    }
}

impl<'a> Eq for Frozen<'a> {}

impl<'a> Hash for Frozen<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();
        let mut left = self.0;
        
        let mut tuple = (self.0, self.1);
        tuple.0 = minmax(tuple.0, tuple.1);
        tuple.1 = maxmin(tuple.0, tuple.1);
        tuple.hash(&mut hasher);
        hasher.finish().hash(state);
    }
}

impl<'a> std::fmt::Display for Frozen<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.left(), self.right())
    }
}

fn minmax<'a>((a, b, _): &'a (i32, i32, i32), (d, e, _): &'a (i32, i32, i32)) -> &'a (i32, i32, i32) {
    if (a, b) < (d, e) {
        &(*a, *b, 0)
    } else {
        &(*d, *e, 0)
    }
}

fn maxmin<'a>((a, b, _): &'a (i32, i32, i32), (d, e, _): &'a (i32, i32, i32)) -> &'a (i32, i32, i32) {
    if (a, b) > (d, e) {
        &(*a, *b, 0)
    } else {
        &(*d, *e, 0)
    }
}

impl<'a> Iterator for Frozen<'a> {
    type Item = &'a (i32, i32, i32);
    

    fn next(&mut self) -> Option<Self::Item> {
        match self.get_axis() {
            0 => {
                if self.0 .0 < self.1 .0 {
                    let ret = self.0;
                    self.0 = &(self.0 .0 + 1, self.0 .1, self.0 .2);
                    Some(ret)
                } else if self.0 .0 > self.1 .0 {
                    let ret = self.1;
                    self.1 = &(self.1 .0 + 1, self.1 .1, self.1 .2);
                    Some(ret)
                } else {
                    None
                }
            }
            1 => {
                if self.0 .1 < self.1 .1 {
                    let ret = self.0;
                    self.0 = &(self.0 .0, self.0 .1 + 1, self.0 .2);
                    Some(ret)
                } else if self.0 .1 > self.1 .1 {
                    let ret = self.1;
                    self.1 = &(self.1 .0, self.1 .1 + 1, self.1 .2);
                    Some(ret)
                } else {
                    None
                }
            }
            2 => {
                if self.0 .2 < self.1 .2 {
                    let ret = self.0;
                    self.0 = &(self.0 .0, self.0 .1, self.0 .2 + 1);
                    Some(ret)
                } else if self.0 .2 > self.1 .2 {
                    let ret = self.1;
                    self.1 = &(self.1 .0, self.1 .1, self.1 .2 + 1);
                    Some(ret)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub struct FrozenIter<'a>(&'a (i32, i32, i32), &'a (i32, i32, i32), usize);

impl<'a> Iterator for FrozenIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.2 {
            0 => {
                self.2 += 1;
                Some(&self.0.0)
            },
            1 => {
                self.2 += 1;
                Some(&self.0.1)
            },
            2 => {
                self.2 += 1;
                Some(&self.0.2)
            },
            3 => {
                self.2 += 1;
                Some(&self.1.0)
            },
            4 => {
                self.2 += 1;
                Some(&self.1.1)
            },
            5 => {
                self.2 += 1;
                Some(&self.1.2)
            },
            _ => None,
        }
    }
}


// use std::cmp::Ordering;

// type Idx = usize;

// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub struct Frozen<'a>(&'a (i32, i32, i32), &'a (i32, i32, i32));

// impl<'a> Frozen<'a> {
//     pub fn new(a: &'a (i32, i32, i32), b: &'a (i32, i32, i32)) -> Self {
//         if a < b {
//             Frozen(a, b)
//         } else {
//             Frozen(b, a)
//         }
//     }
    
//     pub fn is_valid_edge(&self) -> bool {

//         let total = (self.0.0 & 0xFFFF) + (self.0.1 & 0xFFFF) + (self.1.0 & 0xFFFF) + (self.1.1 & 0xFFFF);
//         (4 <= total) && (total <= 10)
//     }

//     pub fn get_axis(&self) -> Idx {
//         // not optimal would like to use indexing (0..2)
//         match (self.0 .0 != self.1 .0, self.0 .1 != self.1 .1, self.0 .2 != self.1 .2) {
//             (true, false, false) => 0,
//             (false, true, false) => 1,
//             (false, false, true) => 2,
//             _ => panic!("VERTS ARE SIMILAR"),
//         }
//     }
// }

// impl<'a> Ord for Frozen<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         let self_sum = self.0 .0.abs()
//             + self.0 .1.abs()
//             + self.0 .2.abs()
//             + self.1 .0.abs()
//             + self.1 .1.abs()
//             + self.1 .2.abs();
//         let other_sum = other.0 .0.abs()
//             + other.0 .1.abs()
//             + other.0 .2.abs()
//             + other.1 .0.abs()
//             + other.1 .1.abs()
//             + other.1 .2.abs();
//         self_sum.cmp(&other_sum)
//     }
// }

// impl<'a> PartialOrd for Frozen<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
