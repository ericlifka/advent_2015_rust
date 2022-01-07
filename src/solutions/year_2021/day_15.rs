use crate::input;
use std::collections::{HashMap,VecDeque};

const LOW_BOUND: usize = 0;
const HIGH_BOUND: usize = 500;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(usize, usize);

impl Point {
    fn from(x: i32, y: i32) -> Option<Point> {
        if x >= LOW_BOUND as i32 && y >= LOW_BOUND as i32 &&
           x < HIGH_BOUND as i32 && y < HIGH_BOUND as i32 {
            Some(Point(x as usize, y as usize))
        } else {
            None
        }
    }

    fn to_coord(&self) -> (i32, i32) {
        let Point(x, y) = self;
        (*x as i32, *y as i32)
    }
}

#[derive(Debug)]
struct Path(Point, u32);

pub fn run() {
    let mut minimum_paths: HashMap<Point, u32> = HashMap::new();
    let mut path_queue: VecDeque<Path> = VecDeque::new();
    let mut cave = [[0u32; HIGH_BOUND]; HIGH_BOUND];
    parse_input(&mut cave);

    path_queue.push_back(Path(Point(0, 0), 0));
    minimum_paths.insert(Point(0, 0), 0);

    while let Some(Path(position, path_risk)) = path_queue.pop_front() {
        if match minimum_paths.get(&position) { None => true, Some(&lowest) => path_risk <= lowest } {
 
            for neighbor in neighbors(&position) {
                if let Some(point) = neighbor {
                    let Point(x, y) = point;
                    let risk = path_risk + cave[y][x];
        
                    if match minimum_paths.get(&point) { None => true, Some(&lowest) => risk < lowest } {
                        minimum_paths.insert(point, risk);
                        path_queue.push_back(Path(point, risk));
                    }
                }
            }
        }
    }

    println!(" part 1: {}", minimum_paths.get(&Point(99, 99)).unwrap());
    println!(" part 2: {}", minimum_paths.get(&Point(499, 499)).unwrap());
}

fn parse_input(cave: &mut [[u32; HIGH_BOUND]; HIGH_BOUND]) {
    let lines = input::read_lines("2021_15")
        .expect("Couldn't read input");

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {

            let mut row_base_risk = lines[y][x..x+1].parse().unwrap();

            for tile_y in 0..5 {
                let mut base_risk = row_base_risk;

                for tile_x in 0..5 {
                    cave[y + tile_y * 100][x + tile_x * 100] = base_risk;

                    rollover_risk(&mut base_risk);
                }

                rollover_risk(&mut row_base_risk);
            }
        }
    }
}

fn rollover_risk(risk: &mut u32) {
    *risk += 1;
    if *risk == 10 {
        *risk = 1;
    }
}

fn neighbors(point: &Point) -> Vec<Option<Point>> {
    let (x, y) = point.to_coord();

    vec![
        Point::from(x+1, y),
        Point::from(x-1, y),
        Point::from(x, y+1),
        Point::from(x, y-1),
    ]
}
