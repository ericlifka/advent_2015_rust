use std::ops::{Sub,Not,Add};
use std::cmp::{Ordering,PartialOrd};
use crate::input;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Copy, Clone, Debug)]
struct PointVector {
    len: i32,
    vec: Point,
    identity: [i32; 3],
    points: [Point; 2],
}

#[derive(Debug)]
struct Scanner {
    name: String,
    beacons: Vec<Point>,
    identity: Vec<PointVector>,
    matches_with_world: usize,
}

impl Not for Point {
    type Output = Self;

    fn not(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Ord for PointVector {
    fn cmp(&self, other: &Self) -> Ordering {
        if self > other {
            Ordering::Greater
        } else if self < other {
            Ordering::Less
        } else {
            Ordering::Equal  
        }
    }

    fn max(self, other: Self) -> Self {
        if other > self {
            other
        } else {
            self
        }
    }

    fn min(self, other: Self) -> Self {
        if other < self {
            other
        } else {
            self
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl PartialOrd for PointVector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    // what a bunch of derp lol
    fn lt(&self, other: &Self) -> bool {
        if self.len < other.len {
            true
        } else if self.len > other.len {
            false
        } else if self.identity[0] < other.identity[0] {
            true
        } else if self.identity[0] > other.identity[0] {
            false
        } else if self.identity[1] < other.identity[1] {
            true
        } else if self.identity[1] > other.identity[1] {
            false
        } else if self.identity[2] < other.identity[2] {
            true
        } else if self.identity[2] > other.identity[2] {
            false
        } else {
            false
        }
    }
    fn le(&self, other: &Self) -> bool {
        self == other || self < other
    }
    fn gt(&self, other: &Self) -> bool {
        self != other && !(self < other)
    }
    fn ge(&self, other: &Self) -> bool {
        !(self < other)
    }
}

impl PartialEq for PointVector {
    fn eq(&self, other: &Self) -> bool {
        self.identity[0] == other.identity[0] &&
        self.identity[1] == other.identity[1] &&
        self.identity[2] == other.identity[2]
    }
}
impl Eq for PointVector {}

impl PointVector {
    fn from_subtract(left: Point, right: Point) -> Self {
        let points = [left, right];
        let vec = left - right;

        let mut identity = [vec.x.abs(), vec.y.abs(), vec.z.abs()];
        identity.sort();

        let len = identity[0] + identity[1] + identity[2];

        PointVector {
            len,
            vec,
            identity,
            points,
        }
    }
}

impl Scanner {
    fn new(name: &str, beacons: Vec<Point>) -> Scanner {
        Scanner {
            name: String::from(name),
            identity: Self::calc_identity(&beacons),
            matches_with_world: 0,
            beacons,
        }
    }

    fn calc_identity(beacons: &Vec<Point>) -> Vec<PointVector> {
        let mut identity: Vec<PointVector> = Vec::new();
        for j in 0..(beacons.len() - 1) {
            for k in (j + 1)..beacons.len() {
                identity.push(PointVector::from_subtract(beacons[j], beacons[k]));
            }
        }

        identity.sort();

        identity
    }

    fn compare_to(&self, other: &Self) -> Vec<(PointVector, PointVector)> {
        let mut matches: Vec<(PointVector, PointVector)> = Vec::new();

        let mut left_vectors = self.identity.iter();
        let mut left_current = left_vectors.next();

        let mut right_vectors = other.identity.iter();
        let mut right_current = right_vectors.next();

        loop {
            match both(left_current, right_current) {
                None => return matches,
                Some((&left, &right)) => {
                    if left < right {
                        left_current = left_vectors.next();
                    } else if right < left {
                        right_current = right_vectors.next();
                    } else {
                        matches.push( (left, right) );

                        left_current = left_vectors.next();
                        right_current = right_vectors.next();
                    }
                }
            }
        }
    }

    fn add_unique_beacons(&mut self, scanner: &Scanner, transformer: Box<dyn Transformer>) {
        let mut unique_beacons: Vec<Point> = Vec::new();

        for &beacon in scanner.beacons.iter() {
            let world_beacon_coord = transformer.transform(beacon);
            let mut should_add = true;

            for &world_beacon in self.beacons.iter() {
                if world_beacon == world_beacon_coord {
                    should_add = false;
                    break;
                }
            }

            if should_add {
                unique_beacons.push(world_beacon_coord);
            }
        }

        for beacon in unique_beacons {
            for &other in self.beacons.iter() {
                self.identity.push(PointVector::from_subtract(beacon, other));
            }
            self.beacons.push(beacon);
        }

        self.identity.sort();
    }
}

trait Transformer {
    fn transform(&self, point: Point) -> Point;
    fn transform_all(&self, points: &Vec<Point>) -> Vec<Point> {
        points.iter().map(|&p| self.transform(p)).collect::<Vec<Point>>()
    }
}

struct Rotation {
    x: fn(Point) -> i32,
    y: fn(Point) -> i32,
    z: fn(Point) -> i32,
}

impl Transformer for Rotation {
    fn transform(&self, p: Point) -> Point {
        Point {
            x: (self.x)(p),
            y: (self.y)(p),
            z: (self.z)(p),
        }
    }
}

impl Rotation {
    fn rotate_to(from: Point, to: Point) -> Box<dyn Transformer> {
        Box::new(Rotation {
            x: Self::transform_accessor(to.x, from),
            y: Self::transform_accessor(to.y, from),
            z: Self::transform_accessor(to.z, from),
        })
    }

    fn transform_accessor(val: i32, point: Point) -> fn(Point) -> i32 {
             if val ==  point.x { |p|  p.x }
        else if val == -point.x { |p| -p.x }
        else if val ==  point.y { |p|  p.y }
        else if val == -point.y { |p| -p.y }
        else if val ==  point.z { |p|  p.z }
        else if val == -point.z { |p| -p.z }
        else {
            panic!("couldn't match target val to point");
        }
    }
}

struct Translation {
    vector: Point
}

impl Transformer for Translation {
    fn transform(&self, p: Point) -> Point {
        p + self.vector
    }
}

impl Translation {
    fn translate_to(vector: Point) -> Box<dyn Transformer> {
        Box::new(Translation {
            vector
        })
    }
}

struct TransformerChain {
    transformers: Vec<Box<dyn Transformer>>
}

impl Transformer for TransformerChain {
    fn transform(&self, p: Point) -> Point {
        let mut transformed = p;
        for t in self.transformers.iter() {
            transformed = t.transform(transformed);
        }

        transformed
    }
}

impl TransformerChain {
    fn new(transformers: Vec<Box<dyn Transformer>>) -> Box<Self> {
        Box::new(Self { transformers })
    }
}

pub fn run() {
    let mut scanners: Vec<Scanner> = get_input();
    let mut global_scanner = scanners.remove(0);
    let mut scanner_locs: Vec<(Point, String)> = vec![
        (Point { x: 0, y: 0, z: 0 }, String::from(&global_scanner.name))
    ];

    while scanners.len() > 0 {
        sort_scanners(&global_scanner, &mut scanners);

        let scanner = scanners.remove(0);
        let mut matches = scanner.compare_to(&global_scanner);
        let m1 = matches.remove(0);
        let m2 = matches.remove(0);
        let mut rotate = Rotation::rotate_to(m1.0.vec, m1.1.vec);
        let s_points = vec![m1.0.points[0], m1.0.points[1], m2.0.points[0], m2.0.points[1]];
        let w_points = vec![m1.1.points[0], m1.1.points[1], m2.1.points[0], m2.1.points[1]];

        let scanner_location = match scanner_location(&w_points, rotate.transform_all(&s_points)) {
            Some(location) => location,
            None => {
                rotate = Rotation::rotate_to(m1.0.vec, !m1.1.vec);

                match scanner_location(&w_points, rotate.transform_all(&s_points)) {
                    Some(location) => location,
                    None => panic!("Couldn't establish rotation")
                }
            }
        };

        scanner_locs.push((scanner_location, String::from(&scanner.name)));
        global_scanner.add_unique_beacons(&scanner, TransformerChain::new(vec![
            rotate,
            Translation::translate_to(scanner_location)
        ]));
    }

    println!("  Part 1: {} beacons", global_scanner.beacons.len());

    let mut max = (0, &String::from("scanner 0"), &String::from("scanner 0"));
    for (s1, s1name) in scanner_locs.iter() {
        for (s2, s2name) in scanner_locs.iter() {
            let len = PointVector::from_subtract(*s1, *s2).len;
            if len > max.0 {
                max = (len, s1name, s2name);
            }
        }
    }

    println!("  Part 2: {} units, between scanner {} and {}", max.0, max.1, max.2);
}

fn both<T>(left: Option<T>, right: Option<T>) -> Option<(T, T)> {
    match left  { None => None, Some(left) =>
    match right { None => None, Some(right) =>
        Some((left, right))
    }}
}

fn scanner_location(global_points: &Vec<Point>,
                    scanner_points: Vec<Point>) -> Option<Point> {

    let opt_1 = global_points[0] - scanner_points[0];
    let opt_2 = global_points[1] - scanner_points[1];
    
    let location = 
        if opt_1 == opt_2 {
            opt_1 
        } else {
            global_points[0] - scanner_points[1]
        };

    let alt_1 = global_points[2] - scanner_points[2];
    let alt_2 = global_points[3] - scanner_points[3];

    let validation = 
        if alt_1 == alt_2 {
            alt_1
        } else {
            global_points[2] - scanner_points[3]
        };

    if location == validation {
        Some(location)
    } else {
        None
    }
}

fn sort_scanners(global_scanner: &Scanner, scanners: &mut Vec<Scanner>) {
    for mut scanner in scanners.iter_mut() {
        scanner.matches_with_world = scanner.compare_to(global_scanner).len();
    }

    scanners.sort_by(|r, l| {
        if l.matches_with_world < r.matches_with_world {
            Ordering::Less
        } else if l.matches_with_world > r.matches_with_world {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn get_input() -> Vec<Scanner> {
    let input = input::read_all("2021_19")
        .expect("couldn't get input");

    let groups: Vec<&str> = input.split("\r\n\r\n").collect();

    let scanners: Vec<Scanner> = groups.iter().map(parse_scanner_group).collect();

    scanners
}

fn parse_scanner_group(group_str: &&str) -> Scanner {
    let lines: Vec<&str> = group_str.split("\r\n").collect();
    let points: Vec<Vec<i32>> = 
        lines[1..].iter()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse().expect("parse error"))
                    .collect::<Vec<i32>>()
            }).collect();

    let beacons: Vec<Point> = points.iter()
        .map(|arr| {
            Point {
                x: arr[0],
                y: arr[1],
                z: arr[2],
            }
        }).collect();

    Scanner::new(lines[0], beacons)
}