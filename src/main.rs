use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src\\2015\\day1.txt")
        .expect("Something went wrong reading the file");

    let mut floor = 0;

    for char in contents.chars() {
        if char == '(' {
            floor += 1;
        }
        else if char == ')' {
            floor -= 1;
        }
    }

    println!("floor = {}", floor);
    println!("Hello, world!");
}
