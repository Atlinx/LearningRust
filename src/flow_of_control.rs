#[allow(dead_code)]
#[allow(unused)]

pub fn main() {
    // 8) Flow of Control

    // 8.1) if/else
    let n = 3;
    if n < 30 {
        println!("n < 30");
    } else {
        println!("n > 30");
    }

    // 8.2) loop
    let mut count = 0u32;
    loop {
        count += 1;

        println!("count: {}", count);
        if count == 3 {
            println!("three");
            break;
        }
    }

    // 8.2.1) Nesting and labels
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
    }
    println!("Exited the outer loop");

    // 8.2.2) Returning from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("breaking from loop value: {}", result);

    // 8.3) while
    let mut n = 1;
    while n < 101 {
        let mut printed = false;
        if n % 15 == 0 {
            print!("fizzbuzz");
            printed = true;
        } else if n % 3 == 0 {
            print!("fizz");
            printed = true;
        } else if n % 5 == 0 {
            print!("buzz");
            printed = true;
        }
        n += 1;
        if printed && n != 101 {
            print!(", ");
        }
    }
    println!();

    // 8.4) for loops
    print!("1-100: ");
    for n in 1..101 {
        print!("{}, ", n);
    }
    println!();

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        // into_iter consumes data
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        // Lets us modify the collection in place
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // 8.5) match
    let number = 13;

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        13..=19 => println!("A teen!"),
        _ => println!("Ain't special"),
    }

    // 8.51) Destructuring
    let tuple = (3, 30, 4);
    match tuple {
        (.., 30) => println!("last element in tuple is 30"),
        (3, ..) => println!("First element in tuple is 3"),
        _ => (),
    }

    let array = [1, -2, 6, 12, -48, 3];
    match array {
        [1, second, tail @ ..] => {
            println!("array[0] == 1, second is {}, rest is {:?}", second, tail)
        }
        _ => (),
    }

    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match reference {
        val => println!(
            "Got a value via destructuring (done automatically by compiler): {:?}",
            val
        ),
    }

    let value = 5;
    match value {
        ref var => println!("got a reference to value: {}", var),
    }
    println!("value after: {}", value);

    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("Added 10, `mut_value`: {}", mut_value);
        }
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    fn test_match(foo: Foo) {
        match foo {
            Foo { x: (1, b), y } => println!("x: (1, {}), y: {}", b, y),
            Foo { y, .. } => println!("y = {}, we don't care about rest", y),
            _ => (),
        };
    }

    test_match(Foo { x: (1, 2), y: 3 });
    test_match(Foo { x: (3, 78), y: 0 });

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }

    // 8.5.3) Binding
    fn age() -> u32 {
        15
    }

    match age() {
        0 => println!("0"),
        n @ 1..=12 => println!("child of age: {}", n),
        n @ 13..=19 => println!("teen of age: {}", n),
        n => println!("old person of age: {}", n),
    }

    // 8.6) if let
    let optional = Some(7);
    if let Some(i) = optional {
        println!("Matched {:?}!", i)
    }
    let letter: Option<i32> = None;
    if let Some(i) = letter {
        println!("Matched letter {:?}!", i)
    } else {
        println!("Letter is empty");
    }

    enum FooEnum {
        Bar,
        Other,
    }
    let a = FooEnum::Bar;
    if let FooEnum::Bar = a {
        println!("a is foobar!");
    }

    // 8.7) let-else
    let array = ["brown", "fox"];
    let ["brown", fox_var] = array else {
      panic!("couldn't match...");
    };

    // 8.8) while let
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("i is {}, try again", i);
            optional = Some(i + 1);
        }
    }
}
