use crate::utils::{break_lines, parse_i32, read_file};

const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

pub fn solution2() {
    let inputs = parse_i32(break_lines(read_file("inputs/day02")));

    let mut safe_reports_pt1 = 0;
    let mut safe_reports_pt2 = 0;
    for line in inputs {
        let is_ascending = line.get(0) < line.get(line.len() - 1);

        let comparison = if is_ascending {
            line.clone()
        } else {
            line.iter().cloned().rev().collect()
        };

        if all_safe_pt1(&comparison) {
            safe_reports_pt1 = safe_reports_pt1 + 1;
            safe_reports_pt2 = safe_reports_pt2 + 1;
        } else if all_safe_pt2(&comparison) {
            safe_reports_pt2 = safe_reports_pt2 + 1;
        }
    }

    print!("Part 1: {} \n", safe_reports_pt1);
    print!("Part 2: {} \n", safe_reports_pt2);
}

fn all_safe_pt1(input: &Vec<i32>) -> bool {
    let sign = input[0] < input[1];
    for i in 0..input.len() - 1 {
        if (input[i] > input[i + 1]) == sign
            || (input[i + 1] - input[i]).abs() < MIN_DIFF
            || (input[i + 1] - input[i]).abs() > MAX_DIFF
        {
            return false;
        }
    }

    return true;
}

fn all_safe_pt2(input: &Vec<i32>) -> bool {
    for i in 0..input.len() {
        if all_safe_pt1(&[&input[..i], &input[i + 1..]].concat()) {
            return true;
        }
    }

    dbg!(input);
    return false;
}
