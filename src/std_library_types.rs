#![allow(unused)]
#![allow(dead_code)]

use std::{boxed, collections::HashMap, mem, vec};

pub fn main() {
    // 19) Std library types
    {
        #[derive(Debug)]
        struct Point {
            x: f64,
            y: f64,
        }
        let point = Point { x: 3.1, y: 4.3 };
        let boxed_point = Box::new(Point { x: 4.3, y: -7.1 });

        println!("point size: {}", mem::size_of_val(&point));
        println!("boxed_point size: {}", mem::size_of_val(&boxed_point));
        println!(
            "boxed_point deref size: {}",
            mem::size_of_val(boxed_point.as_ref())
        );

        let mut mutable_boxed_point = Box::new(Point { x: -38.3, y: 29.3 });
        println!("mutable_boxed_point: {:?}", mutable_boxed_point);
        let borrowed_mut_box_point = mutable_boxed_point.as_mut();
        borrowed_mut_box_point.x = 10.0;
        println!("mutable_boxed_point after mut: {:?}", mutable_boxed_point);
    }

    {
        let collected_iterator: Vec<i32> = (0..10).collect();
        println!("Collected (0..10) into {:?}", collected_iterator);

        let mut x_list = vec![1i32, 2, 3];
        x_list.push(4);
        println!("Fourth element: {}", x_list[3]);
        println!("Vector: {:?}", x_list);

        for (i, x) in x_list.iter().enumerate() {
            println!("[{}]: {}", i, x);
        }

        for x in x_list.iter_mut() {
            *x *= 3;
        }
        println!("Updated vector: {:?}", x_list);
    }

    {
        let pangram = "The quick brown fox jumps over the lazy dog";
        println!("Pangram: {}", pangram);

        println!("Words in reverse");
        for word in pangram.split_whitespace().rev() {
            println!("> {}", word);
        }

        let mut chars: Vec<char> = pangram.chars().collect();
        chars.sort();
        chars.dedup();

        let mut string = String::new();
        for c in chars {
            string.push(c);
            string.push_str(", ");
        }

        println!("string: {}", string);

        let chars_to_trim = [' ', ','];
        let slice_of_chars_to_trim: &[char] = &chars_to_trim;
        let trimmed_str: &str = string.trim_matches(slice_of_chars_to_trim);
        println!("trimmed string: {}", string);
    }

    {
        let mut contacts = HashMap::new();

        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        println!("contacts: {:?}", contacts);
        println!("Ashley: {}", contacts.get("Ashley").unwrap());
    }
}
