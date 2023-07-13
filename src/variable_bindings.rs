#![allow(dead_code)]
#![allow(unused)]

pub fn main() {
    // 4) Variable Bindings
    let an_integer = 1u32;
    let unit: ();

    let copied_integer = an_integer;

    println!(
        "An integer: copied: {:?} original: {:?}",
        copied_integer, an_integer
    );

    let _unused_variable = 3u32;

    // 4.1) Mutability
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // 4.2) Scope
    {
        let my_variable = 30;
        {
            let my_other_variable = 20;
            println!(
                "my_variable: {} my_other_variable: {}",
                my_variable, my_other_variable
            );
        }
        println!("my_variable: {}", my_variable);
    }

    // 4.3) Declare First
    let a_binding;
    {
        let x = 2;
        a_binding = x * 2;
    }
    println!("a binding: {}", a_binding);

    // 4.4) Freezing
    let mut mutable_integer = 7i16;

    {
        let mutable_integer = mutable_integer;
        // Shadowing overwrites mutability
        // mutable_integer = 10;
    }

    mutable_integer += 3;
    println!("mutable_integer: {}", mutable_integer);
}
