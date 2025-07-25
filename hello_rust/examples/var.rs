#![allow(unused)]


fn main() {
    // by default let variables in rust are immutable
    // to make them mutable use `mut`
    // also, you can redeclare a variable in rust

    let x = 1;
    let x: i32 = 1; // same as the above line
    let x: _ = 1; // we tell the compiler to figure out the type itself

    let mut x: i32 = 1; // mutable variable
    x += 1; // we can change the value since the variable is now mutable

    // Shadowing
    let x = 2; // this shadows the previous x, not changes it
    let x: bool = true; // this shadows the previous x, not changes it. 

    //println!
    let x = 1;
    println!("x is : {}", x);
    println!("x is : {x}"); // this is a new syntax in Rust -- inline

    let z: i32 = x + x;
    println!("{x} + {x} = {z}"); // we cannot put (x+x) inside the {}, only simple variables are allowed

    //positional arguments
    println!("{0} + {0} = {1}", x, x + x); // here 0th input is x and 1st input is x+x

    // debug
    println!("DEBUG: x {:?}", x); // {:?} is used for debug formatting, it prints the value in a debug format
    println!("x {:#?}", x); // {:#?} is used for pretty printing, it prints the value in a more readable format
}
