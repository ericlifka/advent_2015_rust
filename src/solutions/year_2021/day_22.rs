use crate::input;
use std::{cmp, collections::VecDeque, ops::{Not, BitAnd, Sub}};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Cuboid {
    x_min: i64, x_max: i64,
    y_min: i64, y_max: i64,
    z_min: i64, z_max: i64,
}

enum Slice { X(i64), Y(i64), Z(i64) }
enum Toggle { On, Off }
struct Instruction(Cuboid, Toggle);

impl Not for Cuboid {
    type Output = bool;

    // True if this cuboid is invalid, False if it's valid
    fn not(self) -> Self::Output {
        self.x_max < self.x_min ||
        self.y_max < self.y_min ||
        self.z_max < self.z_min
    }
}

impl BitAnd for Cuboid {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Self {
            x_min: cmp::max(self.x_min, other.x_min),
            x_max: cmp::min(self.x_max, other.x_max),
            y_min: cmp::max(self.y_min, other.y_min),
            y_max: cmp::min(self.y_max, other.y_max),
            z_min: cmp::max(self.z_min, other.z_min),
            z_max: cmp::min(self.z_max, other.z_max),
        }
    }
}

impl Sub for Cuboid {
    type Output = VecDeque<Self>;

    fn sub(self, other: Self) -> Self::Output {
        let left = self.slice_down(Slice::X(other.x_min - 1));
        let right = self.slice_up(Slice::X(other.x_max + 1));

        let middle = 
            self.slice_up(Slice::X(other.x_min))
                .slice_down(Slice::X(other.x_max));

        let top_middle = middle.slice_up(Slice::Y(other.y_max + 1));
        let bottom_middle = middle.slice_down(Slice::Y(other.y_min - 1));

        let center =
            middle.slice_up(Slice::Y(other.y_min))
                  .slice_down(Slice::Y(other.y_max));

        let center_front = center.slice_down(Slice::Z(other.z_min - 1));
        let center_back = center.slice_up(Slice::Z(other.z_max + 1));

        let mut slices = VecDeque::from([
            left, right, top_middle, bottom_middle, center_front, center_back
        ]);
        
        slices.retain(|&cuboid| !!cuboid);

        slices
    }
}

impl Cuboid {
    fn slice_down(&self, slice: Slice) -> Self {
        match slice {
            Slice::X(at) => Self { x_max: at, ..*self },
            Slice::Y(at) => Self { y_max: at, ..*self },
            Slice::Z(at) => Self { z_max: at, ..*self },
        }
    }

    fn slice_up(&self, slice: Slice) -> Self {
        match slice {
            Slice::X(at) => Self { x_min: at, ..*self },
            Slice::Y(at) => Self { y_min: at, ..*self },
            Slice::Z(at) => Self { z_min: at, ..*self },
        }
    }

    fn area(&self) -> i64 {
        (self.x_max - self.x_min) * (self.y_max - self.y_min) * (self.z_max - self.z_min)
    }
}

pub fn run() {
    let mut existing_cuboids: VecDeque<Cuboid> = VecDeque::new();

    for Instruction(cuboid, toggle) in parse_input() {
        let mut new_cuboids: VecDeque<Cuboid> = VecDeque::new();

        existing_cuboids.retain(|&shape| {
            let overlap = cuboid & shape;
            if !overlap {
                true
            } else {
                if overlap != shape {
                    new_cuboids.append(&mut (shape - overlap));
                }
                false
            }
        });
        
        match toggle {
            Toggle::Off => (),
            Toggle::On => {
                new_cuboids.push_back(cuboid);
            },
        };

        existing_cuboids.append(&mut new_cuboids);
    }

    let area = existing_cuboids.iter().fold(0, |acc, c| acc + c.area());

    println!(" Part 2: {}", area);
}

// input format:
//   on x=30638..51715,y=35872..46823,z=-56349..-29601
//   off x=-27831..-17345,y=-19515..-7960,z=76040..92701
fn parse_input() -> Vec<Instruction> {
    input::read_lines("2021_22")
        .expect("Couldn't read input")
        .iter()
        .map(|line| {
            let ins_split = line.split(" ").collect::<Vec<&str>>();
            let shape = parse_cuboid(ins_split[1]);
            
            Instruction(shape, match ins_split[0] {
                "on" => Toggle::On,
                "off" => Toggle::Off,
                _ => panic!("unknown instruction"),
            })
        })
        .collect()
}

// input format:
//   x=30638..51715,y=35872..46823,z=-56349..-29601
fn parse_cuboid(input: &str) -> Cuboid {
    let coords = input.split(",").collect::<Vec<&str>>();
    let (x_min, x_max) = parse_range(coords[0]);
    let (y_min, y_max) = parse_range(coords[1]);
    let (z_min, z_max) = parse_range(coords[2]);

    Cuboid {
        x_min, x_max,
        y_min, y_max,
        z_min, z_max,
    }
}

// input format:
//   x=-27831..-17345
fn parse_range(input: &str) -> (i64, i64) {
    let parts = input[2..].split("..").collect::<Vec<&str>>();
    let min = parts[0];
    let max = parts[1];

    ( 
        min.parse().expect("parse error"),
        max.parse().expect("parse error"),
    )
}