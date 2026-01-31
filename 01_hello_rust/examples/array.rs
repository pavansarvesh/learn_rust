#![allow(unused)]

use std::array;

// Array - collection of elements with length known at compile time
// Slice - collection of element with length not known at compile time
fn main() {
    // Array
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0]: {}", arr[0]);

    // Write
    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 99;

    let arr: [u32; 10] = [0; 10];
    println!("arr: {:?}", arr);

    // Slice
    let nums: [i32; 10] = [-1, 1, 2, -2, -3, 3, -4, 4, -5, 5];
    // first 3
    let s: &[i32] = &nums[..3];
    println!("{:?}", s);
    // last 3
    let s: &[i32] = &nums[7..];
    println!("{:?}", s);
}
