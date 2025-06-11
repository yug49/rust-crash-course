#![allow(unused)]

pub fn print() {
    println!("my");
}

// Private - cannot be called by main
fn f() {
    println!("private");
}

pub mod a;

// Private module
// Cannot be called outside of this module
mod b;

fn g() {
    b::print();
}

// Go one level up in the module tree
use super::foo;

fn call_foo_print() {
    foo::print();
}
