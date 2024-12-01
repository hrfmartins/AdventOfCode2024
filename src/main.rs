use aoc::solutions::day01;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "day01" => day01::solution1(),
        // "solution2" => solution2(),
        _ => println!("Invalid input"),
    }
}
