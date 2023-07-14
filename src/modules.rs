#![allow(unused)]
#![allow(dead_code)]

mod my_mod {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }

    impl<T: std::fmt::Display> std::fmt::Display for ClosedBox<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ClosedBox: {}", self.contents)
        }
    }

    pub fn public_function() {
        println!("I'm called from a public fn");
    }
    pub mod nested {
        pub fn public_nested_function() {
            println!("I'm called from a nested fn");
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }
}

struct RandomStruct {
    amount: i32,
}

pub fn main() {
    my_mod::public_function();
    my_mod::nested::public_nested_function();

    let open_box = my_mod::OpenBox {
        contents: "public info",
    };
    println!("The open box contains: {}", open_box.contents);
    let closed_box = my_mod::ClosedBox::new("classified info");
    println!("The closed box is: {}", closed_box);
}
