use crate::game::piece::{Piece};
use crate::game::constants::{BOARD_WIDTH, BOARD_HEIGHT};

#[derive(Copy, Clone)]

pub struct Board {
    pub board: [[Option<Team>; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
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

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Team> {
        if x < BOARD_WIDTH as usize && y < BOARD_HEIGHT as usize {
            self.board[y][x]
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

            println!("Placing piece at: {} {}", new_x, new_y);

            self.board[new_y][new_x] = Some(team);
        }
    }

    /*/// Places a piece on the board at the specified coordinates.
    /// This function does not perform any validation and assumes that the coordinates are valid and the piece is not placed on an occupied space.
    pub fn place_piece(&mut self, x: usize, y: usize, team: Team, piece: PieceType) {
        match piece {
            PieceType::Mono => {
                self.board[y][x] = Some(team);
            },
            PieceType::Domino => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
            },
            PieceType::TrioL => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
            },
            PieceType::TrioI => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
            },
            PieceType::TetroO => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
            },
            PieceType::TetroT => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y][x + 2] = Some(team);
            },
            PieceType::TetroI => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 3][x] = Some(team);
            },
            PieceType::TetroL => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
            },
            PieceType::TetroZ => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
            },
            PieceType::PentoL => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 3][x] = Some(team);
                self.board[y + 3][x + 1] = Some(team);
            },
            PieceType::PentoT => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y][x + 2] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
            },
            PieceType::PentoV => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 2][x + 2] = Some(team);
            },
            PieceType::PentoS => {
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y][x + 2] = Some(team);
                self.board[y][x + 3] = Some(team);
            },
            PieceType::PentoZ => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 2][x + 2] = Some(team);
            },
            PieceType::PentoI => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 3][x] = Some(team);
                self.board[y + 4][x] = Some(team);
            },
            PieceType::PentoP => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x] = Some(team);
            },
            PieceType::PentoW => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 2][x + 2] = Some(team);
            },
            PieceType::PentoU => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
                self.board[y][x + 2] = Some(team);
            },
            PieceType::PentoR => {
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
                self.board[y][x + 2] = Some(team);
            },
            PieceType::PentoX => {
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
            },
            PieceType::PentoY => {
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 3][x + 1] = Some(team);
            },
        }
    }*/
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

    pub fn to_string(&self) -> String {
        match self {
            Team::Blue => "BLUE".to_string(),
            Team::Yellow => "YELLOW".to_string(),
            Team::Red => "RED".to_string(),
            Team::Green => "GREEN".to_string(),
        }
    }
}