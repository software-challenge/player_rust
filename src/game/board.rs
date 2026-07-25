use crate::game::pieces::Pieces;

#[derive(Copy, Clone)]

pub struct Board {
    pub board: [[Option<Team>; 20]; 20],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None; 20]; 20],
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

    /// Places a piece on the board at the specified coordinates.
    /// This function does not perform any validation and assumes that the coordinates are valid and the piece is not placed on an occupied space.
    pub fn place_piece(&mut self, x: usize, y: usize, team: Team, piece: Pieces) {
        match piece {
            Pieces::Mono => {
                self.board[y][x] = Some(team);
            },
            Pieces::Domino => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
            },
            Pieces::TrioL => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
            },
            Pieces::TrioI => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
            },
            Pieces::TetroO => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
            },
            Pieces::TetroT => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y][x + 2] = Some(team);
            },
            Pieces::TetroI => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 3][x] = Some(team);
            },
            Pieces::TetroL => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
            },
            Pieces::TetroZ => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
            },
            Pieces::PentoL => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 3][x] = Some(team);
                self.board[y + 3][x + 1] = Some(team);
            },
            Pieces::PentoT => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y][x + 2] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
            },
            Pieces::PentoV => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 2][x + 2] = Some(team);
            },
            Pieces::PentoS => {
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y][x + 2] = Some(team);
                self.board[y][x + 3] = Some(team);
            },
            Pieces::PentoZ => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 2][x + 2] = Some(team);
            },
            Pieces::PentoI => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 2][x] = Some(team);
                self.board[y + 3][x] = Some(team);
                self.board[y + 4][x] = Some(team);
            },
            Pieces::PentoP => {
                self.board[y][x] = Some(team);
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x] = Some(team);
            },
            Pieces::PentoW => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 2][x + 2] = Some(team);
            },
            Pieces::PentoU => {
                self.board[y][x] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
                self.board[y][x + 2] = Some(team);
            },
            Pieces::PentoR => {
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
                self.board[y][x + 2] = Some(team);
            },
            Pieces::PentoX => {
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 1][x + 2] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
            },
            Pieces::PentoY => {
                self.board[y][x + 1] = Some(team);
                self.board[y + 1][x] = Some(team);
                self.board[y + 1][x + 1] = Some(team);
                self.board[y + 2][x + 1] = Some(team);
                self.board[y + 3][x + 1] = Some(team);
            },
        }
    }
}

#[derive(Copy, Clone)]
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