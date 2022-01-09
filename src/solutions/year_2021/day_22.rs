use crate::input;

#[derive(Debug)]
struct Cuboid {
    x_min: i32, x_max: i32,
    y_min: i32, y_max: i32,
    z_min: i32, z_max: i32,
}

enum Overlap {
    None,
    Contains,
    Contained,
    Overlap(Cuboid),
}

impl Cuboid {
    fn compare(&self, other: &Cuboid) -> Overlap {
        Overlap::None
    }

    fn fragment(&self, cutout: &Cuboid) -> Vec<Cuboid> {
        vec![]
    }
}

enum Toggle { On, Off }

#[derive(Debug)]
struct Instruction(Cuboid, Toggle);

#[derive(Debug)]
enum List {
    Nil,
    Node {
        item: Cuboid,
        next: Box<List>,
    },
}

struct ListIterator {
    head: &List,
    cur: Option<&List>,
}

impl List {
    fn next(&self) -> &List {
        switch self {
            Nil => Nil,
            Node { next, .. } => next,
        }
    }
}

impl ListIterator {
    fn new(head: &List) -> ListIterator {
        ListIterator { head, cur: None }
    }

    fn remove(&self) {
        match self.cur {
            None => None,
            Some(List::Nil) => None,
            Some(List::Node { item, next }) => {
                
            }
        }
    }
}

impl Iterator for ListIterator {
    type Item = &Cuboid;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur = match self.cur {
            None => Some(head),
            Some(cur) => Some(cur.next()),
        };

        match self.cur {
            None => None,
            Some(List::Nil) => None,
            Some(List::Node { item, .. }) => item,
        }
    }
}

pub fn run() {
    let mut existing_cuboids = List::Nil;

    for Instruction(cuboid, toggle) in parse_input() {
        let iter = ListIterator::new(existing_cuboids);
        for shape in iter {
            match cuboid.compare(&shape) {
                Overlap::None => None,
                Overlap::Contains => iter.remove(),
                Overlap::Contained => break,
                Overlap::Overlap(overlap) => {
                    iter.remove();
                    for new_cuboid in shape.fragment(overlap) {
                        existing_cuboids = List::Node {
                            item: new_cuboid,
                            next: Box::new(existing_cuboids),
                        };
                    }
                },
            }
        }
        
        match toggle {
            Toggle::Off => None,
            Toggle::On => {
                existing_cuboids = List::Node {
                    item: cuboid,
                    next: Box::new(existing_cuboids),
                };
            },
        };
    }
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
fn parse_range(input: &str) -> (i32, i32) {
    let parts = input[2..].split("..").collect::<Vec<&str>>();
    let min = parts[0];
    let max = parts[1];

    ( 
        min.parse().expect("parse error"),
        max.parse().expect("parse error"),
    )
}