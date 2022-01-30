use crate::input;

struct Point {
    x: usize,
    y: usize,
}

enum Instruction {
    Toggle(Point, Point),
    Off(Point, Point),
    On(Point, Point),
}
use Instruction::*;

impl Point {
    fn from(desc: &str) -> Point {
        let parts: Vec<&str> = desc.split(",").collect();
        Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        }
    }
}

pub fn run() {
    let mut light_grid: Vec<Vec<u16>> = Vec::new();
    for _ in 0..1000 {
        let mut row: Vec<u16> = Vec::new();
        for _ in 0..1000 {
            row.push(0);
        }
        light_grid.push(row);
    }
    let instructions = get_instructions();

    for instruction in instructions {
        match instruction {
            Toggle(start, stop) => {
                for x in start.x..=stop.x {
                    for y in start.y..=stop.y {
                        light_grid[y][x] += 2;
                    }
                }
            },
            Off(start, stop) => {
                for x in start.x..=stop.x {
                    for y in start.y..=stop.y {
                        if light_grid[y][x] > 0 {
                            light_grid[y][x] -= 1;
                        }
                    }
                }
            },
            On(start, stop) => {
                for x in start.x..=stop.x {
                    for y in start.y..=stop.y {
                        light_grid[y][x] += 1;
                    }
                }
            },
        }
    }

    let mut count = 0u64;
    for x in 0..1000 {
        for y in 0..1000 {
            count += light_grid[y][x] as u64;
        }
    }

    println!("Part 2: {}", count);
}

fn get_instructions() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input::read_lines("2015_06").expect("couldn't load input") {
        let tokens: Vec<&str> = line.split(" ").collect();

        instructions.push( match tokens[0] {
            "toggle" => Toggle(Point::from(tokens[1]), Point::from(tokens[3])),
            "turn" =>  match tokens[1] {
                "off" => Off(Point::from(tokens[2]), Point::from(tokens[4])),
                "on" => On(Point::from(tokens[2]), Point::from(tokens[4])),
                _ => panic!("unknown command")
            },
            _ => panic!("unknown command")
        });
    }

    instructions
}