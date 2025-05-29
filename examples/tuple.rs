#![allow(unused)]

// Example of using a tuple to return multiple output
fn return_many() -> (u32, bool) {
    (1, true)
}

// Function returning no value is the same as returning an empty tuple
fn no_return() {}

// Same output as no_return
fn return_empty_tuple() -> () {}

fn main() {
    // Tuples - fixed size, known at compile time
    let t: (bool, char, u32) = (true, 'c', 3);
    println!("({}, {}, {})", t.0, t.1, t.2);

    // Destructure
    let (a, b, c) = t;
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Ignore first and last
    let (_, b, _) = t;

    // Destructure on function output
    let (u, b) = return_many();

    // Empty tuple - Functions with no return value returns an empty tuple
    let empty = ();

    // Nested tuple
    let nested = (('a', 1.23), ('b', true, 1), ());
    println!("nested: {}", (nested.0).1);
}
