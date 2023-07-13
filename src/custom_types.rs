#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Unit;

impl Rectangle {
    fn area(&self) -> f32 {
        rect_area(self)
    }

    fn width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }

    fn height(&self) -> f32 {
        self.top_left.y - self.bottom_right.y
    }
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle {
        top_left: my_top_left,
        bottom_right: my_bottom_right,
    } = rectangle;
    let height = my_top_left.y - my_bottom_right.y;
    let width = my_bottom_right.x - my_top_left.x;
    height * width
}

fn square(top_left: Point, size: f32) -> Rectangle {
    let bottom_right = Point {
        x: top_left.x + size,
        y: top_left.y - size,
    };
    Rectangle {
        top_left,
        bottom_right,
    }
}

enum WebEvent {
    PageLoad,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum EnumWithReallyLongName {
    Add,
    Subtract,
}
type EnumAlias = EnumWithReallyLongName;

impl EnumWithReallyLongName {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}.", x, y)
        }
        _ => (),
    }
}

pub fn main() {
    // 3) Custom Types

    // 3.1) Structures
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 30.3, y: 3.23 };
    let top_left = Point {
        x: 1.0,
        y: point.y + 5.0,
    };
    let bottom_right = Point { x: 5.2, ..point };

    let unit = Unit;
    println!("Unit: {:?}", unit);

    let rectangle = Rectangle {
        top_left,
        bottom_right,
    };
    println!("Rectangle Area: {:?}", rect_area(&rectangle));
    println!("Rectangle: {:?}", rectangle);
    println!("Rectangle Area: {:?}", rectangle.area());
    println!("Rectangle Height: {:?}", rectangle.height());
    println!("Rectangle Width: {:?}", rectangle.width());

    let square = square(Point::new(5.3, 1.3), 5.0);
    println!("Square: {:?}", square);
    println!(
        "Square width: {:?} height: {:?}",
        square.width(),
        square.height()
    );

    // 3.2) Enums
    let pressed = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 20 };
    inspect(pressed);
    inspect(click);

    let x = EnumAlias::Add;
    println!("Run 3 + 50: {}", x.run(3, 50));
    let x = EnumAlias::Subtract;
    println!("Run 3 + 50: {}", x.run(3, 50));

    // 3.2.1) use
    {
        use std::fmt;

        struct MyStruct(i32, bool);

        impl Display for MyStruct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "MyStruct: {} {}", self.0, self.1)
            }
        }

        println!("MyStruct: {}", MyStruct(304, false));
    }

    // 3.2.2) C-like
    {
        enum Number {
            Zero,
            One,
            Two,
        }

        enum Color {
            Red = -3,
            Green = -2,
            Blue = 1,
        }

        println!("zero is {}", Number::Zero as i32);
        println!("one is {}", Number::One as i32);
        println!("red is {}", Color::Red as i32);
    }

    // 3.2.3) Testcase: linked-list
    {
        enum List {
            Cons(u32, Box<List>),
            Nil,
        }

        impl List {
            fn new() -> List {
                List::Nil
            }

            // Use self consumes this list by borrowing it
            fn prepend(self, elem: u32) -> List {
                List::Cons(elem, Box::new(self))
            }

            // Using &self doesn't consume this list
            fn len(&self) -> i32 {
                match self {
                    List::Cons(_, next) => next.len() + 1,
                    List::Nil => 0,
                }
            }

            fn stringify(&self) -> String {
                match self {
                    List::Cons(value, next) => {
                        format!("{} -> {}", value.to_string(), next.stringify())
                    }
                    List::Nil => String::from("Nil"),
                }
            }
        }

        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("Linked list has length: {}", list.len());
        println!("Linked list: {}", list.stringify());
    }

    {
        // 3.3) Constants
        static mut LANGUAGE: &str = "Rust";
        const THRESHOLD: i32 = 10;

        fn is_big(n: i32) -> bool {
            n > THRESHOLD
        }

        let n = 16;

        unsafe {
            // Mutable static variable is unsafe (Likely b/c of multithreading issues)
            println!("This is {}", LANGUAGE);
            println!("The threshold is {}", THRESHOLD);
            println!("{} is {}", n, if is_big(n) { "big " } else { "small" });

            LANGUAGE = "English";
            println!("New language is {}", LANGUAGE);
        }
    }
}
