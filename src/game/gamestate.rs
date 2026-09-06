use crate::game::board::Board;
use crate::game::color::Color;
use crate::game::gamerulelogic;
use crate::game::piece::{Piece, PieceType};
use crate::game::r#move::Move;

/// Holds all information of a games state.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameState {
    starting_piece: PieceType,
    is_starting_team_one: bool,
    board: Board,
    turn: u8,
    round: u8,
    current_turn_color: Color,
    pieces: [Vec<PieceType>; 4] // blue, yellow, red, green
}

impl GameState {

    pub fn new(starting_piece: PieceType, is_starting_team_one: bool, board: Board, turn: u8, round: u8, current_turn_color: Color, blue_pieces: Vec<PieceType>, yellow_pieces: Vec<PieceType>, red_pieces: Vec<PieceType>, green_pieces: Vec<PieceType>) -> Self {
        GameState {
            starting_piece,
            is_starting_team_one,
            board,
            turn,
            round,
            current_turn_color,
            pieces: [blue_pieces, yellow_pieces, red_pieces, green_pieces],
        }
    }

    /// Applies a move to the game state without any validation.
    /// This function assumes that the move is valid and directly updates the game state.
    /// If the move is invalid, this function may lead to an inconsistent game state.
    pub fn apply_move_unchecked(&mut self, m: &Move, turn: u8) {
        self.board.place_piece_unchecked(m.x, m.y, m.color, Piece::new(m.piece, m.rotation, m.is_flipped));

        // Remove used piece from the corresponding color's available pieces
        match m.color {
            Color::Blue => {
                self.pieces[0].retain(|&p| p != m.piece);
            },
            Color::Yellow => {
                self.pieces[1].retain(|&p| p != m.piece);
            },
            Color::Red => {
                self.pieces[2].retain(|&p| p != m.piece);
            },
            Color::Green => {
                self.pieces[3].retain(|&p| p != m.piece);
            },
        }

        self.turn = turn;

        if self.turn.is_multiple_of(4) {
           self.round += 1;
        }

        const COLOR_ORDER_ONE: [Color; 4] = [Color::Blue, Color::Yellow, Color::Red, Color::Green];
        const COLOR_ORDER_TWO: [Color; 4] = [Color::Yellow, Color::Red, Color::Green, Color::Blue];

        // Current color can be caluclated from turn number
        if self.is_starting_team_one {
            self.current_turn_color = COLOR_ORDER_ONE[(self.turn % 4) as usize];
        } else {
           self.current_turn_color = COLOR_ORDER_TWO[(self.turn % 4) as usize];
        }
    }

    /// Applies a move to the game state with validation.
    /// Returns true if the move was valid and applied, false otherwise.
    pub fn apply_move(&mut self, m: &Move, turn: u8) -> bool {

        if gamerulelogic::is_valid_move(&self, m) == false {
            return false;
        }

        self.board.place_piece(m.x, m.y, m.color, Piece::new(m.piece, m.rotation, m.is_flipped));

        // Remove used piece from the corresponding color's available pieces
        match m.color {
            Color::Blue => {
                self.pieces[0].retain(|&p| p != m.piece);
            },
            Color::Yellow => {
                self.pieces[1].retain(|&p| p != m.piece);
            },
            Color::Red => {
                self.pieces[2].retain(|&p| p != m.piece);
            },
            Color::Green => {
                self.pieces[3].retain(|&p| p != m.piece);
            },
        }

        self.turn = turn;

        if self.turn.is_multiple_of(4) {
           self.round += 1;
        }

        const COLOR_ORDER_ONE: [Color; 4] = [Color::Blue, Color::Yellow, Color::Red, Color::Green];
        const COLOR_ORDER_TWO: [Color; 4] = [Color::Yellow, Color::Red, Color::Green, Color::Blue];

        // Current color can be caluclated from turn number
        if self.is_starting_team_one {
            self.current_turn_color = COLOR_ORDER_ONE[(self.turn % 4) as usize];
        } else {
           self.current_turn_color = COLOR_ORDER_TWO[(self.turn % 4) as usize];
        }

        true
    }

    pub fn get_current_turn_color(&self) -> &Color {
        &self.current_turn_color
    }
    
    pub fn set_current_turn_color(&mut self, color: Color) {
        self.current_turn_color = color;
    }

    pub fn get_turn(&self) -> &u8 {
        &self.turn
    }

    pub fn set_turn(&mut self, turn: u8) {
        self.turn = turn;
    }

    pub fn get_round(&self) -> &u8 {
        &self.round
    }

    pub fn set_round(&mut self, round: u8) {
        self.round = round;
    }

    pub fn get_starting_piece(&self) -> &PieceType {
        &self.starting_piece
    }

    pub fn set_starting_piece(&mut self, piece: PieceType) {
        self.starting_piece = piece;
    }

    pub fn is_starting_team_one(&self) -> &bool {
        &self.is_starting_team_one
    }

    pub fn set_is_starting_team_one(&mut self, is_starting_team_one: bool) {
        self.is_starting_team_one = is_starting_team_one;
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn get_color_pieces(&self, color: &Color) -> &[PieceType] {
        match color {
            Color::Blue => &self.pieces[0],
            Color::Yellow => &self.pieces[1],
            Color::Red => &self.pieces[2],
            Color::Green => &self.pieces[3],
        }
    }

    pub fn set_color_pieces(&mut self, color: &Color, pieces: Vec<PieceType>) {
        match color {
            Color::Blue => self.pieces[0] = pieces,
            Color::Yellow => self.pieces[1] = pieces,
            Color::Red => self.pieces[2] = pieces,
            Color::Green => self.pieces[3] = pieces,
        }
    }
}