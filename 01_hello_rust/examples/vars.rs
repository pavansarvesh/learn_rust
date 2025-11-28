#![allow(unused)]
//constant
const NUM: u32 = 45;

fn main() {
    // Variables
    // - Immutable by default
    // - use `mut` keyword to make it mutable

    let mut x = 1;
    x += 1;

    let y = -1;

    // Shadowing
    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;

    let x: _ = 123;
    let z = x + x;
    //printing
    println!("x is {}", x);
    println!("x is {x}");

    //positional argument
    println!("{0} + {0} = {1}", x, x + x)
}
