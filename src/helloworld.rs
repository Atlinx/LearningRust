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

pub fn main() {
    // 1
    println!("Hello, world!");
    println!("I'm a Rustacean 🦀");

    // 1.1
    // Comment
    /*
    Line comment
     */

    // 1.2
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

    // 1.2.1
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

    // 1.2.1
    let pet = Pet {
        name: "year",
        age: 30,
    };
    let tuple_struct = TupleStruct(30, false);
    println!("My Pet: {} My Tuple Struct: {}", pet, tuple_struct);
}
