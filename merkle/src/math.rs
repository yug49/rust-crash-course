use std::cmp::PartialOrd;

pub fn min<A: PartialOrd>(x: A, y: A) -> A {
    if x < y {
        x
    } else {
        y
    }
}
