#![allow(dead_code, unused)]

use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // Associated function
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Method
    fn add(self, ref other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn main() {
    // 9) functions

    // 9.1) Methods
    let mut point = Point::new(5.0, 3.0);
    println!("point: {:?}", point);
    point = point.add(Point::new(7.3, 2.84));
    println!("point after add: {:?}", point);

    // 9.2) Closures
    let out_var = 42;
    let closure = |i: i32| -> i32 { i + 42 };

    println!("closure: {}", closure(32));

    let one = || 1;
    println!("closure returning one: {}", one());

    // 9.2.1) Capturing
    let color = String::from("green");
    let print = || println!("color: {}", color);

    print();
    let _reborrow = &color;
    print();
    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        print!("count: {}", count);
    };
    inc();
    inc();
    let _count_reborroed = &mut count;

    let movable = Box::new(3);
    let consume = || {
        // movable is copied into the closure
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();

    let haystack = vec![1, 2, 3];
    // haystack is moved into the closure
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 9.2.2) As input parameters

    // C#-esque generics
    // Closures must be passed with generics
    fn apply<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }

    apply(|| {
        println!("Hello");
    });

    // 9.2.4) Type anonymity
    // Must use generics to pass closure into function, because closures can bind any
    // variable to themselves which creates an anonymous structure.

    // 9.2.4) Input functions
    // Define a function which takes a generic `F` argument
    // bounded by `Fn`, and calls it
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    // Define a wrapper function satisfying the `Fn` bound
    fn function() {
        println!("I'm a function!");
    }

    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

    // 9.2.5) Output functions
    fn create_print_fn<'a>(text: &'a str) -> impl Fn() + 'a {
        move || println!("This is a: {}", text)
    }

    let text = "hello";
    let print_fn = create_print_fn(text);
    print_fn();
    print_fn();
    println!("Text: {}", text);
}
