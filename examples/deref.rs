#![allow(unused)]

fn main() {
    // Immutable reference
    let x = 10;
    let y = &x;
    let z = *y;
    println!("z: {z}");

    // Mutable reference
    let mut x = 1;
    let y = &mut x;
    *y += 1;
    println!("x: {x}");
}
