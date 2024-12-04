use aoc::solutions;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "day01" => solutions::day01::solution1(),
        "day02" => solutions::day02::solution2(),
        _ => println!("Invalid input"),
    }
}
