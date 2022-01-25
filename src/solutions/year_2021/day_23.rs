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

#[derive(Copy, Clone, PartialEq)]
enum Pod {
    A, B, C, D,
    Empty,
}

#[derive(Copy, Clone, PartialEq)]
enum HomeState {
    Emptying,
    Filling,
    Finished,
}

use Pod::*;
use HomeState::*;

impl Pod {
    fn cost(&self) -> u32 {
        match self {
            Empty => 0,
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn empty(&self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }

    fn occupied(&self) -> bool {
        !self.empty()
    }

    fn occupant(self) -> Option<Pod> {
        match self {
            Empty => None,
            _ => Some(self),
        }
    }
}

#[derive(Copy, Clone)]
struct Board {
    energy: u32,
    space: [Pod; 27],
    a_home: HomeState,
    b_home: HomeState,
    c_home: HomeState,
    d_home: HomeState,
}

impl Board {
    fn move_pod(&self, start: usize, end: usize) -> Option<Board> {
        let moving_pod = self.space[start];
        let mut spaces_moved: u32 = 0;
        let mut from: usize = start;
        let mut to: usize = end;

        if self.space[to].occupied() { // quick abort if the target space isn't empty
            return None;
        }

        if from >= 11 { 
            // count spaces to get out of the home
            let index = from - 11;
            let col = index / 4;
            spaces_moved += ((index as u32) % 4) + 1;
            from = (col + 1) * 2;

            if self.space[from].occupied() {
                return None;
            }
        }

        if to >= 11 { 
            // count spaces to get into target spot in home
            let index = to - 11;
            let col = index / 4;
            spaces_moved += ((index as u32) % 4) + 1;
            to = (col + 1) * 2;
        }

        while from != to {
            if from < to {
                from += 1;
            } else {
                from -= 1;
            }
            spaces_moved += 1;

            if self.space[from].occupied() {
                return None;
            }
        }

        // route is clear, execute swap
        let mut board = *self;

        let energy = moving_pod.cost() * (spaces_moved as u32);
        board.energy += energy;
        board.space[end] = moving_pod;
        board.space[start] = Empty;

        match start {
            14 => if board.space[14] == Empty {
                board.a_home = Filling;
            },
            18 => if board.space[18] == Empty {
                board.b_home = Filling;
            },
            22 => if board.space[22] == Empty {
                board.c_home = Filling;
            },
            26 => if board.space[26] == Empty {
                board.d_home = Filling;
            },
            _ => {}
        }

        match end {
            11 => if board.space[11] == A {
                board.a_home = Finished;
            },
            15 => if board.space[15] == B {
                board.b_home = Finished;
            },
            19 => if board.space[19] == C {
                board.c_home = Finished;
            },
            23 => if board.space[23] == D {
                board.d_home = Finished;
            },
            _ => {}
        }

        Some(board)
    }

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

    fn move_to_home(&self, start: usize) -> Option<Board> {
        self.space[start]
            .occupant()
            .and_then(|pod| self.get_home_slot(pod))
            .and_then(|target| self.move_pod(start, target))
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

    fn get_all_possible_moves(mut board: Board) -> VecDeque<Board> {
        let mut changed = false;

        for i in 0..11 {
            if let Some(new_board) = board.move_to_home(i) {
                board = new_board;
                changed = true;
            }
        }

        for home_type in [A, B, C, D] {
            let move_attempt = board.get_next_to_leave(home_type)
                                    .and_then(|start| board.move_to_home(start));

            if let Some(new_board) = move_attempt {
                board = new_board;
                changed = true;
            }
        }

        let mut moves = VecDeque::new();
        
        for home_type in [A, B, C, D] {
            if let Some(start) = board.get_next_to_leave(home_type) {
                for i in [0, 1, 3, 5, 7, 9, 10] {
                    if let Some(board) = board.move_pod(start, i) {
                        moves.push_back(board);
                    }
                }
            }
        }

        if moves.len() == 0 && changed {
            moves.push_back(board);
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
        2. Some kind of prioritizing of board states
    */

    let mut least_energy_used: u32 = /*u32::MAX*/ 150000;
    let mut board_states: VecDeque<Board> = VecDeque::new();
    board_states.push_back(starting_board());

    while let Some(next_board) = board_states.pop_front() {
        if next_board.energy < least_energy_used {
            for board in Board::get_all_possible_moves(next_board) {
                if board.finished() {
                    if board.energy < least_energy_used {
                        least_energy_used = board.energy;
                    }
                } else {
                    board_states.push_back(board);
                }
            }
        }
    }

    println!("All paths explored\n Least energy spent: {} energy", least_energy_used);
}

fn starting_board() -> Board {
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
