use crate::input;
use std::fmt;

const WIDTH: usize = 139;
const HEIGHT: usize = 137;
type SeaFloor = [[Cell; WIDTH]; HEIGHT];
type MoveMarkers = [[bool; WIDTH]; HEIGHT];

fn get_y(y: usize) -> usize {
    if y >= HEIGHT { 0 } else { y }
}
fn get_x(x: usize) -> usize {
    if x >= WIDTH { 0 } else { x }
}


#[derive(Debug, Copy, Clone)]
enum Cell {
    South,
    East,
    Empty,
}
use Cell::*;

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            South => "v",
            East => ">",
            Empty => ".",
        })
    }
}

pub fn run() {
    println!("2021:25");
    // return calc_input_size();

    let mut sea_floor: SeaFloor = [[Empty; WIDTH]; HEIGHT];
    let mut to_move: MoveMarkers = [[false; WIDTH]; HEIGHT];
    fill_sea_floor(&mut sea_floor);

    print_sea_floor(&sea_floor);
    let mut keep_going = true;
    let mut i = 0;

    while keep_going {
        i+= 1;
        keep_going = false;

        if move_east(&mut sea_floor, &mut to_move) {
            keep_going = true;
        }
        if move_south(&mut sea_floor, &mut to_move) {
            keep_going = true;
        }

        // println!("step {}", i);
        // print_sea_floor(&sea_floor);
    }

    print_sea_floor(&sea_floor);
    println!("step {}", i);
}

fn move_south(sea_floor: &mut SeaFloor, to_move: &mut MoveMarkers) -> bool {
    reset_move_markers(to_move);
    let mut had_moves = false;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let South = sea_floor[y][x] {
                if let Empty = sea_floor[get_y(y + 1)][x] {
                    to_move[y][x] = true;
                    had_moves = true;
                }
            }
        }
    }

    if !had_moves {
        return false;
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if to_move[y][x] {
                sea_floor[get_y(y + 1)][x] = South;
                sea_floor[y][x] = Empty;
                to_move[y][x] = false;
            }
        }
    }

    true
}

fn move_east(sea_floor: &mut SeaFloor, to_move: &mut MoveMarkers) -> bool {
    reset_move_markers(to_move);
    let mut had_moves = false;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let East = sea_floor[y][x] {
                if let Empty = sea_floor[y][get_x(x + 1)] {
                    to_move[y][x] = true;
                    had_moves = true;
                }
            }
        }
    }

    if !had_moves {
        return false;
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if to_move[y][x] {
                sea_floor[y][get_x(x + 1)] = East;
                sea_floor[y][x] = Empty;
                to_move[y][x] = false;
            }
        }
    }

    true
}

fn reset_move_markers(markers: &mut MoveMarkers) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            markers[y][x] = false;
        }
    }
}

fn fill_sea_floor(sea_floor: &mut SeaFloor) {
    let lines = input::read_lines("2021_25").expect("couldn't load input");

    for y in 0..lines.len() {
        let line = lines[y].as_bytes();

        for x in 0..line.len() {
            sea_floor[y][x] = match line[x] {
                b'>' => East,
                b'v' => South,
                  _  => Empty,
            };
        }
    }
}

fn print_sea_floor(floor: &SeaFloor) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", floor[y][x]);
        }
        println!("");
    }
    println!("");
}

fn calc_input_size() {
    let lines = input::read_lines("2021_25").expect("couldn't load input");
    println!("Width: {}, Height: {}", lines[0].len(), lines.len());
}
