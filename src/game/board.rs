use crate::game::{
    color::Color,
    piece::{Piece},
    constants::BOARD_SIZE
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    board: [[Option<Color>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// Returns the number of tiles of the given color on the board.
    /// Calculates the count every time it is called, which may be inefficient for frequent calls.
    pub fn get_colored_tiles(&self, color: &Color) -> u32 {
        let mut num = 0;
        for row in self.board.iter() {
            for cell in row.iter() {
                if let Some(cell_color) = cell {
                    if cell_color == color {
                        num += 1;
                    }
                }
            }
        }
        num
    }

    /// Returns the Color of the given cell (x, y) on the board, or None if the cell is empty or out of bounds.
    pub fn get_cell(&self, x: usize, y: usize) -> Option<Color> {
        if x < BOARD_SIZE && y < BOARD_SIZE {
            self.board[y][x]
        } else {
            None
        }
    }

    /// Sets the Color of the given cell (x, y) on the board and returns true if the cell was within bounds, false otherwise.
    /// Does not perform any validation and assumes that the coordinates are valid.
    pub fn set_cell(&mut self, x: usize, y: usize, color: Color) -> bool {
        if x < BOARD_SIZE && y < BOARD_SIZE {
            self.board[y][x] = Some(color);
            return true;
        } 
        false
    }

    /// Place the specified piece on the board at the given coordinates (x, y) for the specified color.
    /// Does not perform any validation and assumes that the coordinates are valid and the piece is not placed on an occupied space.
    pub fn place_piece_unchecked(&mut self, x: usize, y: usize, color: Color, piece: Piece) {
        for coord in piece.get_coordinates() {
            let new_x = x + coord.x as usize;
            let new_y = y + coord.y as usize;

            self.board[new_y][new_x] = Some(color);
        }
    }

    /// Place the specified piece on the board at the given coordinates (x, y) for the specified color.
    /// Returns true if the piece was placed successfully, false otherwise.
    pub fn place_piece(&mut self, x: usize, y: usize, color: Color, piece: Piece) -> bool {
        
        let mut checked_coords: Vec<(usize, usize)> = vec![];

        for coord in piece.get_coordinates() {
            let new_x = x + coord.x as usize;
            let new_y = y + coord.y as usize;

            if new_x >= BOARD_SIZE || new_y >= BOARD_SIZE || self.board[new_y][new_x].is_some() {
                return false;
            }

            checked_coords.push((new_x, new_y));
        }

        for (new_x, new_y) in checked_coords {
            self.board[new_y][new_x] = Some(color);
        }

        true
    }

    /// Prints the board to the console.
    pub fn print_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Some(color) => {
                        match color {
                            Color::Blue => print!("B "),
                            Color::Yellow => print!("Y "),
                            Color::Red => print!("R "),
                            Color::Green => print!("G "),
                        }
                    },
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}