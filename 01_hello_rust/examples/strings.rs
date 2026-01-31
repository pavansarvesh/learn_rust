#![allow(unused)]

fn main() {
    let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let length = msg.len();
    let s: &str = &msg[0..4];
    println!("s = {}", s);

    let s: &str = "Hello World";
}
