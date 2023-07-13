#![allow(dead_code)]
#![allow(unused)]

use std::{fmt, mem};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", (self.0, self.1))?;
        write!(f, "{:?}", (self.2, self.3))
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    return Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
}

pub fn main() {
    // 2) Primitives
    let logical: bool = true;
    let mut inferred_type = 12;
    inferred_type = 30;

    let inferred_type = 203;

    // 2.1) Literals and Operators
    let scientific = 7.6e-4;
    let one = 0b1101u32;
    let two = 0b0101u32;
    println!("{:0>4b} AND {:0>4b} {:0>4b}", one, two, one & two);
    println!("{:0>4b} OR {:0>4b} {:0>4b}", one, two, one | two);
    println!("{:0>4b} XOR {:0>4b} {:0>4b}", one, two, one ^ two);

    // 2.2) Tuples
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    // 2.3) Arrays
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 20] = [4; 20];

    println!("First element of array: {}", xs[0]);
    println!("Second element of array: {}", xs[1]);

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    for i in 0..ys.len() + 1 {
        // One element to far
        match ys.get(i) {
            Some(yval) => println!("{}: {}", i, yval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
