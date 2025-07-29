#![allow(unused)]

// Scalar - data types that represent a single value

fn main() {
    // singed int
    // range: -(2^(n-1)) to 2^(n-1) - 1
    let i0: i8 = -1; // n = 8
    let i1: i16 = 2; // n = 16
    let i2: i32 = 3;
    let i3: i64 = -4;
    let i4: i128 = 5;

    // unsigned integers
    // range: 0 to 2^n - 1
    let u0: u8 = 1; // n= 8
    let u1: u16 = 2;
    let u2: u32 = 3;
    let u3: u64 = 4;
    let u4: u128 = 5;

    // dependes on computer architecture
    let i5: isize = -6; // 32 if 32 bits computer and 64 if 64 bits computer
    let u5: usize = 6; // -----"-----

    // floating point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // boolean
    let b: bool = true;

    // Characters - we use '', if "" is used then it is a string not a char
    let c: char = 'x';

    // type conversions
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("{i} as u32 is {u}");

    // min and max
    let i_min = i32::MIN;
    let u_max = u32::MAX;

    println!("i32 min is: {i_min}");
    println!("u32 max is: {u_max}");
}