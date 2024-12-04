use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|x| x.unwrap()).collect()
}

pub fn break_lines(input: Vec<String>) -> Vec<Vec<String>> {
    input
        .iter()
        .map(|x| x.split_whitespace().map(String::from).collect())
        .collect()
}

pub fn parse_i32(input: Vec<Vec<String>>) -> Vec<Vec<i32>> {
    input
        .iter()
        .map(|x| x.iter().map(|y| y.parse::<i32>().unwrap()).collect())
        .collect()
}
