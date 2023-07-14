#![allow(dead_code)]
#![allow(unused)]

pub fn main() {
    // 15) Scoping rules
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);

    // 15.3) Borrowing
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alic"),
        age: Box::new(20),
    };
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // println!("The person struct is {:?}", person);
    println!("The person's age from person struct is {}", person.age);

    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // eat_box_i32(boxed_i32);
        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);

    // 15.3.3) The ref pattern
    {
        #[derive(Debug, Clone, Copy)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut point = Point { x: 0, y: 0 };
        let _copy_of_x = {
            let Point {
                x: ref mut ref_to_x,
                y: _,
            } = point;
            ref_to_x
        };
        // println!("Point: {:?}", point);
        // point.x = 5;
        println!("_copy_of_x: {:?}", _copy_of_x);
    }

    // 15.4) Lifetimes
    {
        fn failed_borrow<'a>() {
            let _x = 12;

            // ERROR: `_x` does not live long enough
            // let y: &'a i32 = &_x;
            // Attempting to use the lifetime `'a` as an explicit type annotation
            // inside the function will fail because the lifetime of `&_x` is shorter
            // than that of `y`. A short lifetime cannot be coerced into a longer one.
        }

        // 15.4.7) Coercion
        // Here, Rust infers a lifetime that is as short as possible.
        // The two references are then coerced to that lifetime.
        fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
            first * second
        }

        // `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
        // Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
        fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
            first
        }

        fn generate_value<'a>() -> &'a i32 {
            &15
        }

        let first = 2;
        let test: &i32;
        {
            let second = 3;
            println!("The product is {}", multiply(&first, &second));
            // test = choose_first(&first, &second);
            test = generate_value();
            println!("{} is the first", test);
        }
        println!("test: {:?}", test);

        static NUM: i32 = 18;

        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }

        {
            let lifetime_num = 9;
            let coereced_static = coerce_static(&lifetime_num);
            println!("coerced_static: {}", coereced_static);
        }
        // Can't use coereced static outside of here
        // println!("coerced_static: {}", coereced_static);
        println!("NUM: {} stays accessible", NUM);

        // 'static in a type means the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.
        fn print_it(input: impl std::fmt::Debug + 'static) {
            println!("'static value passed in is: {:?}", input);
        }

        let i = 5;
        print_it(i);
        // Using a reference mean it now has a lifetime
        // print_it(&i);
    }
}
