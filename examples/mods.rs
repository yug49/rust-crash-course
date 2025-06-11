#![allow(unused)]

use rust_crash_course::my::a::print as a_print;
use rust_crash_course::{foo, my};

fn main() {
    foo::print();
    my::print();
    my::a::print();
    a_print();
    let s = my::a::build("rust".to_string());
}
