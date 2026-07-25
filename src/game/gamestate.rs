use crate::game::board::Board;
use crate::game::pieces::Pieces;
use crate::game::r#move::Move;

#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub turn: u8,
    pub blue_pieces: Vec<Pieces>,
    pub yellow_pieces: Vec<Pieces>,
    pub red_pieces: Vec<Pieces>,
    pub green_pieces: Vec<Pieces>,
}

impl GameState {
    /// Applies a move to the game state without any validation.
    /// This function assumes that the move is valid and directly updates the game state.
    /// If the move is invalid, this function may lead to an inconsistent game state.
    pub fn apply_move_unchecked(&mut self, m: &Move) {
        self.board.place_piece(m.x, m.y, m.team, m.piece);
        self.turn += 1;
    }
}