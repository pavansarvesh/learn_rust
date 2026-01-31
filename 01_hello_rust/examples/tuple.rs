#![allow(unused)]

fn main() {
    let t: (bool, char, u32) = (true, 'a', 1);
    println!("{},{},{}", t.0, t.1, t.2);

    // empty tuple = unit type
    let t = ();

    // Nested tuple
    let nested = (('a', 1, 1.23), (true, 1u32, -1i32), ());
    println!("nested.0.1 : {}", (nested.0).1);

    // Destructing a tuple
    let t: (bool, char, u32) = (true, 'a', 1);
    let (a, b, c) = t;
    println!("a = {}, b = {}, c = {}", a, b, c)
}
