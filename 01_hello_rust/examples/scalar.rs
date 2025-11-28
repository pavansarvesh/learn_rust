#![allow(unused)]

// Scalar - datatype that represent a single value
fn main() {
    // Signed integers
    // Range: -(2^(n-1)) to 2^(n-1) - 1

    // i8  →  -2^(8-1)  to  2^(8-1) - 1
    let i0: i8 = -1;

    // i16 →  -2^(16-1) to  2^(16-1) - 1
    let i1: i16 = 2;

    // i32 →  -2^(32-1) to  2^(32-1) - 1
    let i2: i32 = 3;

    // i64 →  -2^(64-1) to  2^(64-1) - 1
    let i3: i64 = -4;

    // i128 → -2^(128-1) to 2^(128-1) - 1
    let i4: i128 = 5;

    // -------------------
    // Type Conversion
    // -------------------

    // A signed 32-bit integer with value -1
    let i: i32 = -1;

    // Casting the signed value to an unsigned 32-bit integer.
    // This does NOT "make it positive." Rust simply reinterprets the bits.
    // -1 in two's complement → 0xFFFF_FFFF → 4294967295 in u32.
    let u: u32 = i as u32;

    // Prints: "-1 as u32 = 4294967295"
    println!("{i} as u32 = {u}");

    // -------------------
    // Min and Max values
    // -------------------

    // Maximum value an i32 can store:  2^31 - 1 = 2147483647
    let i_max = i32::MAX;
    let u_max = u32::MAX;

    // Minimum value a u32 can store:   0
    let i_min = i32::MIN;
    let u_min = u32::MIN;

    println!("i32 max: {i_max}");
    println!("i32 min: {i_min}");
    println!("u32 max: {u_max}");
    println!("u32 min: {u_min}");
}