#![allow(unused)]
#![allow(dead_code)]

pub fn main() {
    // 17) macro_rules
    {
        macro_rules! say_hello {
            () => {
                println!("Hello!")
            };
        }
        say_hello!();
    }

    // 17.1) Syntax
    {
        macro_rules! create_function {
            ($func_name:ident) => {
                fn $func_name() {
                    println!("You called {:?}()", stringify!($func_name));
                }
            };
        }

        create_function!(foo);
        create_function!(bar);

        macro_rules! print_result {
            ($expression:expr) => {
                println!("{:?} = {:?}", stringify!($expression), $expression);
            };
        }

        foo();
        bar();

        print_result!(1u32 + 1);
        print_result!({
            let x = 1u32;
            x * x + 2 * x - 1
        });
    }

    // 17.1.2) Overload
    {
        // `test!` will compare `$left` and `$right`
        // in different ways depending on how you invoke it:
        macro_rules! test {
            // Arguments don't need to be separated by a comma.
            // Any template can be used!
            ($left:expr; and $right:expr) => {
                println!(
                    "{:?} and {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left && $right
                )
            };
            // ^ each arm must end with a semicolon.
            ($left:expr; or $right:expr) => {
                println!(
                    "{:?} or {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left || $right
                )
            };
        }

        test!(1i32 + 1 == 2*32; and 2i32 * 2 == 4i32);
        test!(true; or false);
    }

    // 17.1.3) Repeat
    {
        macro_rules! find_min {
            // Base case
            ($x:expr) => ($x);
            // Recursive step
            ($x:expr, $($y:expr),+) => {
                std::cmp::min($x, find_min!($($y),+))
            };
        }

        println!("Min: {}", find_min!(1));
        println!("Min: {}", find_min!(1 + 2, 2));
        println!("Min: {}", find_min!(5, 2 * 3, 4, 7));
    }

    {
        // 17.3) DSL (Domain Specific Languages)
        macro_rules! calculate {
            (eval $e:expr) => {
                let val: usize = $e;
                println!("{} = {}", stringify!($e), val)
            };
        }

        println!("DSL:");
        calculate! {
            eval 1 + 2
        };
        calculate! {
            eval (1 + 2) * (3 / 4)
        };
    }

    {
        // 17.4) Variadics
        macro_rules! calculate {
            // Base case
            (eval $e:expr) => {
                let val: usize = $e;
                println!("{} = {}", stringify!($e), val)
            };
            // Recursive step
            (eval $e:expr, $(eval $es:expr),+) => {
                calculate! { eval $e }
                calculate! { $(eval $es),+ }
            };
        }

        println!("Variadic:");
        calculate! {
            eval 1 + 2,
            eval 3 + 4,
            eval (2 * 3) + 1
        }
    }
}
