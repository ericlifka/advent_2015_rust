use crate::input;
use std::collections::{HashMap,VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point(usize, usize);

impl Point {
    fn from(x: i32, y: i32) -> Option<Point> {
        if x >= 0 && y >= 0 && x < 100 && y < 100 {
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
    let mut cave = [[0u32; 100]; 100];
    parse_input(&mut cave);

    path_queue.push_back(Path(Point(0, 0), 0));
    minimum_paths.insert(Point(0, 0), 0);


    while let Some(Path(position, path_risk)) = path_queue.pop_front() {
        if match minimum_paths.get(&position) { None => true, Some(&lowest) => path_risk <= lowest } {
 
            for point in neighbors(&position) {
                let Point(x, y) = point;
                let risk = path_risk + cave[y][x];
    
                if match minimum_paths.get(&point) { None => true, Some(&lowest) => risk < lowest } {
                    minimum_paths.insert(point, risk);
                    path_queue.push_back(Path(point, risk));
                }
            }
        }
    }

    println!("minimum path to end: {}", minimum_paths.get(&Point(99, 99)).unwrap());
}

fn parse_input(cave: &mut [[u32; 100]; 100]) {
    let lines = input::read_lines("2021_15")
        .expect("Couldn't read input");

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            cave[y][x] = lines[y][x..x+1].parse().unwrap();
        }
    }
}

fn neighbors(point: &Point) -> Vec<Point> {
    let (x, y) = point.to_coord();

    let neighbors = vec![
        Point::from(x+1, y),
        Point::from(x-1, y),
        Point::from(x, y+1),
        Point::from(x, y-1),
    ];
    
    neighbors
        .iter()
        .filter(|n| match n { Some(_) => true, None => false })
        .map(|n| n.unwrap())
        .collect()
        // .map(|(x, y): &(i32, i32)| -> Point { Point(x as usize, y as usize) })
}

// fn go_north(Point(x, y): Point) -> Option<Point> {
//     if y > 0 {
//         Some(Point(x, y - 1))
//     } else {
//         None
//     }
// }