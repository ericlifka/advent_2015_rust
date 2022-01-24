use std::fmt::{Display,Formatter,Result};
/*
sample:
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########

input:
#############
#...........#
###B#A#A#D###
  #D#C#B#A#
  #D#B#A#C#
  #B#C#D#C#
  #########

  BBDA
  DCBA
  DBAC
  CADC

#############
#...........#
###B#B#D#A###
  #D#C#B#A#
  #D#B#A#C#
  #C#A#D#C#
  #########
*/

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pod {
    A, B, C, D,
    Empty,
}

impl Display for Pod {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Pod::*;
        write!(f, "{}", match self {
            Empty => " ",
            A => "A",
            B => "B",
            C => "C",
            D => "D",
        })
    }
}

impl Pod {
    fn cost(&self) -> u32 {
        use Pod::*;

        match self {
            Empty => 0,
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn empty(&self) -> bool {
        use Pod::*;

        match self {
            Empty => true,
            A | B | C | D => false,
        }
    }

    fn occupied(&self) -> bool {
        !self.empty()
    }
}

#[derive(Copy, Clone, Debug)]
struct Board {
    space: [Pod; 27],
    energy: u32,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "\n╔═══════════╗\n║e:{:#9}║\n╠═══════════╣\n║{}{}{}{}{}{}{}{}{}{}{}║\n╚═╣{}║{}║{}║{}╠═╝\n  ║{}║{}║{}║{}║\n  ║{}║{}║{}║{}║\n  ║{}║{}║{}║{}║\n  ╚═╩═╩═╩═╝\n", 
            self.energy,
        
            self.space[0], self.space[1], self.space[2], self.space[3], self.space[4], self.space[5],
            self.space[6], self.space[7], self.space[8], self.space[9], self.space[10],

            self.space[11], self.space[15], self.space[19], self.space[23],
            self.space[12], self.space[16], self.space[20], self.space[24],
            self.space[13], self.space[17], self.space[21], self.space[25],
            self.space[14], self.space[18], self.space[22], self.space[26],
        )
    }
}

impl Board {
    fn move_pod(&self, start: usize, end: usize) -> Option<Board> {
        use Pod::*;

        let moving_pod = self.space[start];
        let mut spaces_moved = 0;
        let mut from = start;
        let mut to = end;

        if let Empty = moving_pod { // can't move if there's no pod there
            println!("target starting space was empty");
            return None;
        }
        if from == to {
            println!("starting and ending space were the same");
            return None;
        }
        if from < 11 && to < 11 { // a piece can't move from the hallway to another spot in the hallway
            println!("start and end both in hallway");
            return None;
        }
        if self.space[to].occupied() { // quick abort if the target space isn't empty
            println!("target space wasn't empty");
            return None;
        }
        match to {
            2 | 4 | 6 | 8 => { // can't stop outside a home
                println!("tried to stop outside a room");
                return None;
            },
            _ => {}
        }

        if from >= 11 { 
            // if start in a home, walk out of the home making sure the path is clear
            let index = from - 11;
            let col = index / 4;
            // let mut offset = index % 4 - 1; // -1 to skip the starting space which isn't empty

            for offset in 0..index % 4 {
            // while offset >= 0 {
                if self.space[ col * 4 + offset + 11 ].occupied() {
                    println!("ran into pod on the way out of room");
                    return None;
                }

                spaces_moved += 1;
                // offset -= 1;
            }

            from = (col + 1) * 2;
            spaces_moved += 1;
        }

        if to >= 11 { 
            // if end in a home, walk into the home making sure the path is clear
            // and make sure there aren't any types in the home that don't match the home type
            let index = to - 11;
            let col = index / 4;
            let target = index % 4;

            let target_type = match col {
                0 => A, 1 => B, 2 => C, 3 => D,
                _ => panic!("unexpected column")
            };

            if moving_pod != target_type { // can only move into their own home
                println!("pod trying to go into the wrong room");
                return None;
            }

            for offset in 0..4 {
                if offset <= target {   // check spaces up through the target space to make sure they're empty
                    spaces_moved += 1;

                    if self.space[ col * 4 + offset + 11 ].occupied() {
                        println!("ran into occupied space in target room");
                        return None;
                    }
                } else {                // spaces after the target space need to be only pod types that belong here
                    if self.space[ col * 4 + offset + 11 ] != target_type {
                        println!("room has pods of the wrong type in it, can't move in.");
                        return None;
                    }
                }
            }

            to = (col + 1) * 2;
        }

        if from == to { // can't move between different spots within the same home
            println!("start and end both within the same room");
            return None;
        }

        let forward = from < to;

        while from != to {
            if forward {
                from += 1;
            } else {
                from -= 1;
            }
            spaces_moved += 1;

            if self.space[from].occupied() {
                println!("Ran into occupied space in hallway");
                return None;
            }
        }

        // route is clear, execute swap
        let mut board = *self;

        let energy = moving_pod.cost() * spaces_moved;
        board.energy += energy;
        board.space[end] = moving_pod;
        board.space[start] = Empty;

        println!("Moved {} from {} to {} and spent {} energy", moving_pod, start, end, energy);

        Some(board)
    }

    fn get_all_possible_moves(&self) -> Vec<Board> {
        let mut moves = Vec::new();

        for start in 0..27 {
            if self.space[start].occupied() {
                for end in 0..27 {
                    if start != end {
                        if let Some(new_position) = self.move_pod(start, end) {
                            moves.push(new_position);
                        }
                    }
                }
            }
        }

        moves
    }
}

pub fn run() {
    println!("2021:23");
    let board = starting_board();

    println!("{}", board);

    let opening_moves = board.get_all_possible_moves();

    for board in opening_moves.iter() {
        println!("{}", board);
    }
    println!("\nFound {} opening moves", opening_moves.len());
}

fn starting_board() -> Board {
    use Pod::*;
    Board {
        space: [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            B, D, D, B,
            A, C, B, C,
            A, B, A, D,
            D, A, C, C,
        ],
        energy: 0,
    }
}
