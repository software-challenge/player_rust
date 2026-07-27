use crate::game::board::{Board, Team};
use crate::game::piece::{Piece, PieceType};
use crate::game::r#move::Move;

#[derive(Clone)]
pub struct GameState {
    pub starting_piece: PieceType,
    pub board: Board,
    pub turn: u8,
    pub round: u8,
    blue_pieces: Vec<PieceType>,
    yellow_pieces: Vec<PieceType>,
    red_pieces: Vec<PieceType>,
    green_pieces: Vec<PieceType>,
}

impl GameState {

    pub fn new(starting_piece: PieceType, board: Board, turn: u8, round: u8, blue_pieces: Vec<PieceType>, yellow_pieces: Vec<PieceType>, red_pieces: Vec<PieceType>, green_pieces: Vec<PieceType>) -> Self {
        GameState {
            starting_piece,
            board,
            turn,
            round,
            blue_pieces,
            yellow_pieces,
            red_pieces,
            green_pieces,
        }
    }

    /// Applies a move to the game state without any validation.
    /// This function assumes that the move is valid and directly updates the game state.
    /// If the move is invalid, this function may lead to an inconsistent game state.
    pub fn apply_move_unchecked(&mut self, m: &Move) {
        self.board.place_piece(m.x, m.y, m.team, Piece { piece_type: m.piece, is_flipped: m.is_flipped, rotation: m.rotation });
        self.turn += 1;
    }

    pub fn get_team_pieces(&self, team: &Team) -> &[PieceType] {
        match team {
            Team::Blue => &self.blue_pieces,
            Team::Yellow => &self.yellow_pieces,
            Team::Red => &self.red_pieces,
            Team::Green => &self.green_pieces,
        }
    }
}