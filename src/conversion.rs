#![allow(dead_code)]
#![allow(unused)]

use std::convert::From;
use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
struct MyNumber {
    value: i32,
}

impl From<i32> for MyNumber {
    fn from(value: i32) -> Self {
        MyNumber { value }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    // We get to declare our own error type for if the cast fails
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius: {}", self.radius)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRadiusError;

impl FromStr for Circle {
    type Err = ParseRadiusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let radius = s.strip_prefix("radius: ").ok_or(ParseRadiusError)?;
        let radius_fromstr = radius.parse::<i32>().map_err(|_| ParseRadiusError)?;
        Ok(Circle {
            radius: radius_fromstr,
        })
    }
}

pub fn main() {
    // 6) Conversion

    let my_str = "hello";
    let my_string = String::from(my_str);

    // 6.1) From and Into
    let num = MyNumber::from(32);
    println!("My number is {:?}", num);
    let int = 32i32;

    // into
    let num: MyNumber = int.into();
    println!("My number is {:?}", num);
    // You can also just use from
    let num = MyNumber::from(int);
    println!("My number is {:?}", num);

    // 6.2) TryFrom and TryInto
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // 6.3) To and from Strings
    let circle = Circle { radius: 5 };
    print!("{} {}", circle, circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let to_circle = "radius: 32".parse::<Circle>().unwrap();

    println!(
        "parsed: {}, turbo_parsed: {}, to_circle: {}",
        parsed, turbo_parsed, to_circle
    );
}
