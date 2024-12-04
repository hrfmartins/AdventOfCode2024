use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solution1() {
    let mut numbers1 = vec![0];
    let mut numbers2 = vec![0];

    let file = File::open("inputs/Day01").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let split_line: Vec<String> = line.unwrap().split_whitespace().map(String::from).collect();

        if split_line.len() < 2 {
            break;
        }

        numbers1.push(split_line.get(0).unwrap().parse::<i32>().unwrap());
        numbers2.push(split_line.get(1).unwrap().parse::<i32>().unwrap());
    }

    numbers1.sort();
    numbers2.sort();

    let mut diff = 0;

    for i in 1..numbers1.len() {
        diff = diff + (numbers2.get(i).unwrap() - numbers1.get(i).unwrap()).abs();
    }

    print!("Part 1: {} \n", diff);

    // For part 2
    //
    let mut similarity_score = 0;
    for i in 1..numbers1.len() {
        let num = numbers1.get(i).unwrap();
        similarity_score =
            similarity_score + num * numbers2.iter().filter(|&x| *x == *num).count() as i32;
    }

    print!("Part 2: {} \n", similarity_score)
}
