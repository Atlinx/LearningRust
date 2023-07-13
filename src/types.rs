#![allow(dead_code)]
#![allow(unused)]
#![allow(overflowing_literals)]

use std::mem;

pub fn main() {
    // 5) Types

    // 5.1) Casting
    let decimal = 65.4321f32;
    let integer = decimal as u8;
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is : {}", 1000 as u8);

    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }

    // 5.2) Literals
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    println!("size of i32 {}", mem::size_of::<i32>());
    println!("size of 3 * i32 {}", mem::size_of::<[i32; 3]>());

    // 5.3) Inference
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);

    println!("vec: {:?}", vec);

    // 5.4) ALiasing

    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 4;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
