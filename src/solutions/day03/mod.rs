use crate::utils::read_file;
use regex::Regex;

pub fn solution3() {
    let inputs = read_file("inputs/Day03");

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let single_digit_match = Regex::new(r"\d{1,3}").unwrap();
    let result = inputs.iter().flat_map(|line| {
        re.find_iter(line).map(|m| {
            let digits_parsed = single_digit_match
                .find_iter(m.as_str())
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            digits_parsed.get(0).unwrap() * digits_parsed.get(1).unwrap()
        })
    });

    println!("{}", result.sum::<i32>());
}
