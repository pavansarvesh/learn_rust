#![allow(unused)]

//Overflow - dosen't panic when ccompiled with --release
fn main() {
    let mut x = u32::MAX;
    x += 1;

    println!("u32 max : {} || x : {}",u32::MAX,x);
}
