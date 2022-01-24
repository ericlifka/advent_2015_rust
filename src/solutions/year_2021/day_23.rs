use std::fmt::{Display,Formatter,Result};
use std::collections::VecDeque;

/*
Plays puzzle game with board like this:
╔═══════════╗
║e:        0║
╠═══════════╣
║           ║
╚═╣B║A║A║D╠═╝
  ║D║C║B║A║
  ║D║B║A║C║
  ║B║C║D║C║
  ╚═╩═╩═╩═╝

array index reference:
╔═════════════════════════════════╗
║ 0  1  2  3  4  5  6  7  8  9 10 ║
╚════╗ 11 ╔╗ 15 ╔╗ 19 ╔╗ 23 ╔═════╝
     ║ 12 ║║ 16 ║║ 20 ║║ 24 ║
     ║ 13 ║║ 17 ║║ 21 ║║ 25 ║
     ║ 14 ║║ 18 ║║ 22 ║║ 26 ║
     ╚════╩╩════╩╩════╩╩════╝
*/

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pod {
    A, B, C, D,
    Empty,
}
use Pod::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum HomeState {
    Emptying,
    Filling,
    Finished,
}
use HomeState::*;

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
    energy: u32,
    space: [Pod; 27],
    a_home: HomeState,
    b_home: HomeState,
    c_home: HomeState,
    d_home: HomeState,
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
            // println!("target starting space was empty");
            return None;
        }
        if from == to {
            // println!("starting and ending space were the same");
            return None;
        }
        if from < 11 && to < 11 { // a piece can't move from the hallway to another spot in the hallway
            // println!("start and end both in hallway");
            return None;
        }
        if self.space[to].occupied() { // quick abort if the target space isn't empty
            // println!("target space wasn't empty");
            return None;
        }
        match to {
            2 | 4 | 6 | 8 => { // can't stop outside a home
                // println!("tried to stop outside a room");
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
                if self.space[ col * 4 + offset + 11 ].occupied() {
                    // println!("ran into pod on the way out of room");
                    return None;
                }

                spaces_moved += 1;
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
                // println!("pod trying to go into the wrong room");
                return None;
            }

            for offset in 0..4 {
                if offset <= target {   // check spaces up through the target space to make sure they're empty
                    spaces_moved += 1;

                    if self.space[ col * 4 + offset + 11 ].occupied() {
                        // println!("ran into occupied space in target room");
                        return None;
                    }
                } else {                // spaces after the target space need to be only pod types that belong here
                    if self.space[ col * 4 + offset + 11 ] != target_type {
                        // println!("room has pods of the wrong type in it, can't move in.");
                        return None;
                    }
                }
            }

            to = (col + 1) * 2;
        }

        if from == to { // can't move between different spots within the same home
            // println!("start and end both within the same room");
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
                // println!("Ran into occupied space in hallway");
                return None;
            }
        }

        // route is clear, execute swap
        let mut board = *self;

        let energy = moving_pod.cost() * spaces_moved;
        board.energy += energy;
        board.space[end] = moving_pod;
        board.space[start] = Empty;

        // might be able to narrow both of these once smarter move finding is in place and there are fewer scenarios for
        // the last piece out or the last piece in.
        if start >= 11 {
            match start {
                11 | 12 | 13 | 14 => {
                    if (board.space[11] == Empty || board.space[11] == A) && 
                       (board.space[12] == Empty || board.space[12] == A) && 
                       (board.space[13] == Empty || board.space[13] == A) && 
                       (board.space[14] == Empty || board.space[14] == A) {
                        board.a_home = Filling;
                    }
                },
                15 | 16 | 17 | 18 => {
                    if (board.space[15] == Empty || board.space[15] == B) && 
                       (board.space[16] == Empty || board.space[16] == B) && 
                       (board.space[17] == Empty || board.space[17] == B) && 
                       (board.space[18] == Empty || board.space[18] == B) {
                        board.b_home = Filling;
                    }
                },
                19 | 20 | 21 | 22 => {
                    if (board.space[19] == Empty || board.space[19] == C)  && 
                       (board.space[20] == Empty || board.space[20] == C)  && 
                       (board.space[21] == Empty || board.space[21] == C)  && 
                       (board.space[22] == Empty || board.space[22] == C)  {
                        board.c_home = Filling;
                    }
                },
                23 | 24 | 25 | 26 => {
                    if (board.space[23] == Empty || board.space[23] == D) && 
                       (board.space[24] == Empty || board.space[24] == D) && 
                       (board.space[25] == Empty || board.space[25] == D) && 
                       (board.space[26] == Empty || board.space[26] == D) {
                        board.d_home = Filling;
                    }
                },
                _ => {}
            }
        }
        
        if end >= 11 {
            match moving_pod {
                A => if board.space[11] == A && board.space[12] == A && board.space[13] == A && board.space[14] == A {
                    board.a_home = Finished;
                },
                B => if board.space[15] == B && board.space[16] == B && board.space[17] == B && board.space[18] == B {
                    board.b_home = Finished;
                },
                C => if board.space[19] == C && board.space[20] == C && board.space[21] == C && board.space[22] == C {
                    board.c_home = Finished;
                },
                D => if board.space[23] == D && board.space[24] == D && board.space[25] == D && board.space[26] == D {
                    board.d_home = Finished;
                },
                Empty => {}
            }
        }
        

        // println!("Moved {} from {} to {} and spent {} energy", moving_pod, start, end, energy);

        Some(board)
    }
/*
Plays puzzle game with board like this:
╔═══════════╗
║e:        0║
╠═══════════╣
║           ║
╚═╣B║A║A║D╠═╝
  ║D║C║B║A║
  ║D║B║A║C║
  ║B║C║D║C║
  ╚═╩═╩═╩═╝

array index reference:
╔═════════════════════════════════╗
║ 0  1  2  3  4  5  6  7  8  9 10 ║
╚════╗ 11 ╔╗ 15 ╔╗ 19 ╔╗ 23 ╔═════╝
     ║ 12 ║║ 16 ║║ 20 ║║ 24 ║
     ║ 13 ║║ 17 ║║ 21 ║║ 25 ║
     ║ 14 ║║ 18 ║║ 22 ║║ 26 ║
     ╚════╩╩════╩╩════╩╩════╝ */
    // fn pod_can_move(&self, index: usize) -> bool {
    //     // quick check to verify a pod can move before spending the time to find all of its destinations
    //     self.space[index].occupied() && match index {
    //         0  => self.space[ 1].empty(),
    //         1  => self.space[11].empty() || self.space[ 3].empty(),
    //         2  => false, // can't be here
    //         3  => self.space[11].empty() || self.space[15].empty() || self.space[ 5].empty(),
    //         4  => false, // can't be here
    //         5  => self.space[ 3].empty() || self.space[15].empty() || self.space[19].empty() || self.space[ 7].empty(),
    //         6  => false, // can't be here
    //         7  => self.space[ 5].empty() || self.space[19].empty() || self.space[23].empty(),
    //         8  => false, // can't be here
    //         9  => self.space[ 7].empty() || self.space[23].empty(),
    //         10 => self.space[ 9].empty(),
    //         11 => self.space[ 1].empty() || self.space[ 3].empty(),
    //         12 => self.space[11].empty() &&(self.space[ 1].empty() || self.space[ 3].empty()),
    //         13 => self.space[12].empty() && self.space[11].empty() &&(self.space[ 1].empty() || self.space[ 3].empty()),
    //         14 => self.space[13].empty() && self.space[12].empty() && self.space[11].empty() &&(self.space[ 1].empty() || self.space[ 3].empty()),
    //         15 => self.space[ 3].empty() || self.space[ 5].empty(),
    //         16 => self.space[15].empty() &&(self.space[ 3].empty() || self.space[ 5].empty()),
    //         17 => self.space[16].empty() && self.space[15].empty() &&(self.space[ 3].empty() || self.space[ 5].empty()),
    //         18 => self.space[17].empty() && self.space[16].empty() && self.space[15].empty() &&(self.space[ 3].empty() || self.space[ 5].empty()),
    //         19 => self.space[ 5].empty() || self.space[ 7].empty(),
    //         20 => self.space[19].empty() &&(self.space[ 5].empty() || self.space[ 7].empty()),
    //         21 => self.space[20].empty() && self.space[19].empty() &&(self.space[ 5].empty() || self.space[ 7].empty()),
    //         22 => self.space[21].empty() && self.space[20].empty() && self.space[19].empty() &&(self.space[ 5].empty() || self.space[ 7].empty()),
    //         23 => self.space[ 7].empty() || self.space[ 9].empty(),
    //         24 => self.space[23].empty() &&(self.space[ 7].empty() || self.space[ 9].empty()),
    //         25 => self.space[24].empty() && self.space[23].empty() &&(self.space[ 7].empty() || self.space[ 9].empty()),
    //         26 => self.space[25].empty() && self.space[24].empty() && self.space[23].empty() &&(self.space[ 7].empty() || self.space[ 9].empty()),
    //         _ => false,
    //     }
    // }

    fn get_home_slot(&self, pod: Pod) -> Option<usize> {
        let (state, indexes) = match pod {
            A => (self.a_home, (11..15).rev()),
            B => (self.b_home, (15..19).rev()),
            C => (self.c_home, (19..23).rev()),
            D => (self.d_home, (23..27).rev()),
            Empty => {return None},
        };
        
        if state == Filling {
            for i in indexes {
                if self.space[i].empty() {
                    return Some(i);
                }
            }
        }

        None
    }

    fn get_next_to_leave(&self, home_type: Pod) -> Option<usize> {
        let (state, indexes) = match home_type {
            A => (self.a_home, 11..15),
            B => (self.b_home, 15..19),
            C => (self.c_home, 19..23),
            D => (self.d_home, 23..27),
            Empty => {return None},
        };

        if state == Emptying {
            for i in indexes {
                if self.space[i].occupied() {
                    return Some(i);
                }
            }
        }

        None
    }

    fn get_all_possible_moves(&self) -> VecDeque<Board> {
        let mut moves = VecDeque::new();

        for i in 0..11 {
            if let Some(target) = self.get_home_slot(self.space[i]) {
                if let Some(board) = self.move_pod(i, target) {
                    moves.push_back(board);
                }
            }
        }

        for home_type in [A, B, C, D] {
            if let Some(start) = self.get_next_to_leave(home_type) {
                if let Some(target) = self.get_home_slot(self.space[start]) {
                    if let Some(board) = self.move_pod(start, target) {
                        moves.push_back(board);
                    }
                }
            }
        }

        if moves.len() == 0 {
            for home_type in [A, B, C, D] {
                if let Some(start) = self.get_next_to_leave(home_type) {
                    for i in 0..11 {
                        if let Some(board) = self.move_pod(start, i) {
                            moves.push_back(board);
                        }
                    }
                }
            }
        }

        moves
    }

    fn finished(&self) -> bool {
        self.a_home == Finished && self.b_home == Finished && self.c_home == Finished && self.d_home == Finished
    }
}

pub fn run() {

    /* ideas:
        1. detect if board in locked state
        2. smarter move detection: don't just try and put each piece into every possible slot
        3. Some kind of prioritizing of board states
        4. precalculated home states to improve knowing if a pod can move into their home and to improve checking if finished
        5. Set a reasonable max energy to cutoff states at
    */

    // let finished = finished_board();
    // let start = starting_board();
    // println!("does it see a done board: {}", finished.finished());
    // println!("{}", finished);
    // println!("does it see an unfished board: {}", start.finished());
    // println!("{}", start);

    // let mut test_board = Board {
    //     space: [
    //         Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
    //         B, D, D, B,
    //         A, C, B, C,
    //         A, B, A, D,
    //         D, A, C, C,
    //     ],
    //     energy: 0,
    // };

    // test_board = test_board.move_pod(11, 0).unwrap();
    // test_board = test_board.move_pod(23, 1).unwrap();

    // let boards = test_board.get_all_possible_moves();

    // for board in boards.iter() {
    //     println!("{}", board);
    // }
    // println!("{} moves found", boards.len());

    let mut least_energy_used: u32 = 1500000;
    let mut max_energy_seen: u32 = 0;
    let mut board_states: VecDeque<Board> = VecDeque::new();
    board_states.push_back(starting_board());

    while let Some(next_board) = board_states.pop_front() {
        // println!("{:#8}", board.energy);
        if next_board.energy >= least_energy_used {
            continue;
        }

        let new_boards = next_board.get_all_possible_moves();
        // let found = new_boards.len();

        for board in new_boards {
            if board.energy > max_energy_seen {
                max_energy_seen = board.energy;
                // println!("{:#8}", max_energy_seen);
            }
            if board.finished() {
                println!("Found a finished board! {}", board.energy);

                if board.energy < least_energy_used {
                    least_energy_used = board.energy;
                    println!("\n**************\n**\n** New lowest: {}\n**\n**************\n", board.energy);
                }
            } else if board.energy < least_energy_used {
                board_states.push_back(board);
            }
        }
        // println!("found {:#8} - queue: {}", found, board_states.len());
    }

    println!("All paths explored\n Least energy spent: {} energy\n Most energy seen: {}", least_energy_used, max_energy_seen);
}

fn starting_board() -> Board {
    use Pod::*;

    Board {
        energy: 0,
        space: [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            B, D, D, B,
            A, C, B, C,
            A, B, A, D,
            D, A, C, C,
        ],
        a_home: Emptying,
        b_home: Emptying,
        c_home: Emptying,
        d_home: Emptying,
    }
}

fn finished_board() -> Board {
    Board {
        energy: 0,
        space: [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            A, A, A, A,
            B, B, B, B,
            C, C, C, C,
            D, D, D, D,
        ],
        a_home: Finished,
        b_home: Finished,
        c_home: Finished,
        d_home: Finished,
    }
}