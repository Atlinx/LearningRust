#![allow(dead_code)]
#![allow(unused)]

pub fn main() {
    // 7) Expressions
    let x = {
        let a = 5;
        let b = a + 5;

        a * b
    };

    println!("x: {}", x);
}
