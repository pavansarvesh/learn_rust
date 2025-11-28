#![allow(unused)]

fn add(x: u32, y: u32) {
    print!("sum : {0}\n", x + y);
}

// with return
fn add_with_return(x: u32, y: u32) -> u32 {
    return x + y;
}

// with return multiple
fn add_with_return_multiple(x: u32, y: u32) -> (u32, bool) {
    return (x + y,true);
}

fn main() {
    println!("Calling `add` function...");
    add(10, 10);
    println!("Calling `add_with_return` function...");
    print!("sum : {}\n", add_with_return(10, 10));
    println!("Calling `add_with_return_multiple` function...");
    print!("sum : {:#?}\n", add_with_return_multiple(10, 10));
}

//.to_string