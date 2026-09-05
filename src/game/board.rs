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

    /// Returns the Team of the given cell (x, y) on the board, or None if the cell is empty or out of bounds.
    pub fn get_cell(&self, x: usize, y: usize) -> Option<Team> {
        if x < BOARD_WIDTH as usize && y < BOARD_HEIGHT as usize {
            self.board[y][x]
        } else {
            None
        }
    }

    /// Sets the Team of the given cell (x, y) on the board and returns true if the cell was within bounds, false otherwise.
    /// Does not perform any validation and assumes that the coordinates are valid.
    pub fn set_cell(&mut self, x: usize, y: usize, team: Team) -> bool {
        if x < BOARD_WIDTH as usize && y < BOARD_HEIGHT as usize {
            self.board[y][x] = Some(team);
            return true;
        } 
        false
    }

    /// Place the specified piece on the board at the given coordinates (x, y) for the specified team.
    /// Does not perform any validation and assumes that the coordinates are valid and the piece is not placed on an occupied space.
    pub fn place_piece_unchecked(&mut self, x: usize, y: usize, team: Team, piece: Piece) {
        for coord in piece.get_coordinates() {
            let new_x = x + coord.x as usize;
            let new_y = y + coord.y as usize;

            self.board[new_y][new_x] = Some(team);
        }
    }

    /// Place the specified piece on the board at the given coordinates (x, y) for the specified team.
    /// Returns true if the piece was placed successfully, false otherwise.
    pub fn place_piece(&mut self, x: usize, y: usize, team: Team, piece: Piece) -> bool {
        
        let mut checked_coords: Vec<(usize, usize)> = vec![];

        for coord in piece.get_coordinates() {
            let new_x = x + coord.x as usize;
            let new_y = y + coord.y as usize;

            if new_x >= BOARD_WIDTH as usize || new_y >= BOARD_HEIGHT as usize || self.board[new_y][new_x].is_some() {
                return false;
            }

            checked_coords.push((new_x, new_y));
        }

        for (new_x, new_y) in checked_coords {
            self.board[new_y][new_x] = Some(team);
        }

        true
    }

    /// Prints the board to the console.
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