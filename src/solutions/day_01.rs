use std::fs;

pub fn run() {
    println!("Day 1");

    let input = fs::read_to_string("input/day_01.txt").expect("input file not found");

    println!("  part 1: {}", count_floor(&input));
    println!("  part 2: {}", enters_basement(&input));
}

fn count_floor(input: &str) -> i32 {
    let mut count: i32 = 0;

    for c in input.chars() {
        count += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
    }

    count
}

fn enters_basement(input: &str) -> usize {
    let mut count: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        count += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };

        if count < 0 {
            return i + 1;
        }
    }

    panic!("Never entered basement");
}