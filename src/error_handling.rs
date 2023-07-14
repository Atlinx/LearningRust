#![allow(dead_code)]
#![allow(unused)]

pub fn main() {
    // 18) Error handling
    {
        fn drink(beverage: &str) {
            if beverage == "lemonade" {
                panic!("AAAAa!!");
            }
            println!("Some refreshing {} is all I need.", beverage);
        }

        drink("water");
        // drink("lemonade");
    }

    {
        // 18.2) abort and unwind
        fn drink(beverage: &str) {
            // You shouldn't drink too much sugary beverages.
            if beverage == "lemonade" {
                if cfg!(panic = "abort") {
                    println!("This is not your party. Run!!!!");
                } else {
                    println!("Spit it out!!!!");
                }
            } else {
                println!("Some refreshing {} is all I need.", beverage);
            }
        }

        drink("water");
        // drink("lemonade");
    }

    {
        // 18.3) Option & unwrap
        fn give_adult(drink: Option<&str>) {
            match drink {
                Some("lemonade") => println!("Yuck! Too sugary"),
                Some(inner) => println!("{}? How nice.", inner),
                None => println!("No drink? Oh well."),
            }
        }

        fn drink(drink: Option<&str>) {
            // `unwrap` returns a `panic` when it receives a `None`.
            let inside = drink.unwrap();
            if inside == "lemonade" {
                panic!("AAAaaaaa!!!!");
            }

            println!("I love {}s!!!!!", inside);
        }

        let water = Some("water");
        let lemonade = Some("lemonade");
        let void = None;

        give_adult(water);
        give_adult(lemonade);
        give_adult(void);

        let coffee = Some("coffee");
        // let nothing = None;

        drink(coffee);
        // drink(nothing);
    }

    {
        #[derive(Debug)]
        enum Food {
            Apple,
            Carrot,
            Potato,
        }

        #[derive(Debug)]
        struct Peeled(Food);
        #[derive(Debug)]
        struct Chopped(Food);
        #[derive(Debug)]
        struct Cooked(Food);

        // Peeling food. If there isn't any, then return `None`.
        // Otherwise, return the peeled food.
        fn peel(food: Option<Food>) -> Option<Peeled> {
            match food {
                Some(food) => Some(Peeled(food)),
                None => None,
            }
        }

        // Chopping food. If there isn't any, then return `None`.
        // Otherwise, return the chopped food.
        fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
            match peeled {
                Some(Peeled(food)) => Some(Chopped(food)),
                None => None,
            }
        }

        // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
        fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
            chopped.map(|Chopped(food)| Cooked(food))
        }

        // A function to peel, chop, and cook food all in sequence.
        // We chain multiple uses of `map()` to simplify the code.
        fn process(food: Option<Food>) -> Option<Cooked> {
            food.map(|f| Peeled(f))
                .map(|Peeled(f)| Chopped(f))
                .map(|Chopped(f)| Cooked(f))
        }

        // Check whether there's food or not before trying to eat it!
        fn eat(food: Option<Cooked>) {
            match food {
                Some(food) => println!("Mmm. I love {:?}", food),
                None => println!("Oh no! It wasn't edible."),
            }
        }

        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));
        // Let's try the simpler looking `process()` now.
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }

    {
        // 18.4) Result
        use std::num::ParseIntError;

        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            let first_number = first_number_str.parse::<i32>()?;
            let second_number = second_number_str.parse::<i32>()?;

            Ok(first_number * second_number)
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        print(multiply("30", "430"));
        print(multiply("30", "hello"));
    }

    {
        // 18.5) Multiple error types
        use std::num::ParseIntError;

        fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
            let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

            opt.map_or(Ok(None), |r| r.map(Some))
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers));
        println!("The first doubled is {:?}", double_first(empty));
        println!("The first doubled is {:?}", double_first(strings));
    }

    {
        use std::error;
        use std::fmt;

        // Change the alias to `Box<error::Error>`.
        type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

        #[derive(Debug, Clone)]
        struct EmptyVec;

        impl fmt::Display for EmptyVec {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        impl error::Error for EmptyVec {}

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            vec.first()
                .ok_or_else(|| EmptyVec.into()) // Converts to Box
                .and_then(|s| {
                    s.parse::<i32>()
                        .map_err(|e| e.into()) // Converts to Box
                        .map(|i| 2 * i)
                })
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("The first doubled is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}
