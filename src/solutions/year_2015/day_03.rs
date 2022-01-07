use crate::input;
use std::collections::HashSet;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Self(x1, y1) = self;
        let Self(x2, y2) = other;
        
        Self(
            x1 + x2,
            y1 + y2,
        )
    }
}

pub fn run() {
    let input = input::read_all("03").expect("input file not found");

    let mut year1_visited: HashSet<Point> = HashSet::new();
    let mut year1_position = Point(0, 0);

    let mut year2_visited: HashSet<Point> = HashSet::new();
    let mut year2_santa = Point(0, 0);
    let mut year2_robot = Point(0, 0);

    year1_visited.insert(year1_position);
    year2_visited.insert(year2_santa);

    for (i, d) in input.chars().enumerate() {
        let direction = move_in(&d);

        year1_position = year1_position + direction;
        year1_visited.insert(year1_position);

        if 0 == i % 2 {
            year2_santa = year2_santa + direction;
            year2_visited.insert(year2_santa);
        } else {
            year2_robot = year2_robot + direction;
            year2_visited.insert(year2_robot);
        }
    }

    println!(" Part 1: {}", year1_visited.len());
    println!(" Part 2: {}", year2_visited.len());
}

fn move_in(direction: &char) -> Point {
    match direction {
        '^' => Point(0, 1),
        '>' => Point(1, 0),
        'v' => Point(0, -1),
        '<' => Point(-1, 0),
        _ => Point(0, 0),
    }
}