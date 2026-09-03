use crate::game::piece::{Piece};
use crate::game::constants::{BOARD_WIDTH, BOARD_HEIGHT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    board: [[Option<Team>; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Team> {
        if x < BOARD_WIDTH as usize && y < BOARD_HEIGHT as usize {
            self.board[y][x]
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, team: Team) -> Option<Team> {
        if x < BOARD_WIDTH as usize && y < BOARD_HEIGHT as usize {
            let previous = self.board[y][x];
            self.board[y][x] = Some(team);
            previous
        } else {
            None
        }
    }

    /// Place the specified piece on the board at the given coordinates (x, y) for the specified team.
    /// Does not perform any validation and assumes that the coordinates are valid and the piece is not placed on an occupied space.
    pub fn place_piece(&mut self, x: usize, y: usize, team: Team, piece: Piece) {
        for coord in piece.get_coordinates() {
            let new_x = x + coord.x as usize;
            let new_y = y + coord.y as usize;

            println!("Placing piece block at: {} {}", new_x, new_y);

            self.board[new_y][new_x] = Some(team);
        }
    }

    pub fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Some(team) => {
                        match team {
                            Team::Blue => print!("B "),
                            Team::Yellow => print!("Y "),
                            Team::Red => print!("R "),
                            Team::Green => print!("G "),
                        }
                    },
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum Team {
    Blue,
    Yellow,
    Red,
    Green,
}

impl Team {
    pub fn from_string(s: &str) -> Self {
        match s {
            "BLUE" => Team::Blue,
            "YELLOW" => Team::Yellow,
            "RED" => Team::Red,
            "GREEN" => Team::Green,
            _ => panic!("Unknown team color: {}", s),
        }
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::Blue => write!(f, "BLUE"),
            Team::Yellow => write!(f, "YELLOW"),
            Team::Red => write!(f, "RED"),
            Team::Green => write!(f, "GREEN"),
        }
    }
}