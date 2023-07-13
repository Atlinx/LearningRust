#![allow(dead_code)]

use std::fmt;

// 1.2.1
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct DeepPrintable {
    printable: DebugPrintable,
    number: i32,
    boolean: bool,
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    last_name: &'a str,
    age: u8,
}

struct Pet<'a> {
    name: &'a str,
    age: u8,
}

struct TupleStruct(i32, bool);

impl<'a> fmt::Display for Pet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pet: \"{}\", {}", self.name, self.age)?;
        Ok(())
    }
}

impl fmt::Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)?;
        Ok(())
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({0} {1} {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}",
            self.red, self.green, self.blue
        )
    }
}

pub fn main() {
    // 1) Hello World
    println!("Hello, world!");
    println!("I'm a Rustacean 🦀");

    // 1.1) Comments
    // Comment
    /*
    Line comment
     */

    // 1.2) Formatted print
    print!("{} days\n", 31);
    println!("{0}, {1}", "Brown", "Fox");
    println!(
        "Named args: Brown: {brown} Fox {fox}",
        fox = "VarFox",
        brown = "VarBrown"
    );

    let number: f64 = 1.0;
    let width: usize = 20;
    println!("{number:>width$}");

    println!("My name is {0}, {1}, {0}", "Bond", "James");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    // 1.2.1) Debug
    let debug_printable = DebugPrintable(50);
    let deep_printable = DeepPrintable {
        printable: DebugPrintable(30),
        number: 30,
        boolean: true,
    };

    println!("{:?} Months in a year", 12);
    println!("DebugPrintable: {:?}", debug_printable);
    println!("DeepPrintable: {:?}", deep_printable);

    let name = "Peter";
    let last_name = "Smith";
    let age = 37;
    let peter = Person {
        name,
        last_name,
        age,
    };
    println!("Pretty print peter: {:#?}", peter);

    // 1.2.1) Display
    let pet = Pet {
        name: "year",
        age: 30,
    };
    let tuple_struct = TupleStruct(30, false);
    println!("My Pet: {} My Tuple Struct: {}", pet, tuple_struct);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Diplay: {}", complex);
    println!("Debug: {:?}", complex);

    // 1.2.2.1) Testcase : List
    let list: List;
    list = List(vec![3, 4, 5]);
    println!("list: {}", list);

    // 1.2.3) Formatting
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}
