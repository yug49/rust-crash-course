#![allow(unused)]

fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y;
}

fn add(x: u32, y: u32) -> u32 {
    println!("x = {x}");
    println!("y = {y}");
    x + y
}

fn print(s: String) {
    println!("{s}{s}{s}{s}{s}");
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    print("😊".to_string());
}