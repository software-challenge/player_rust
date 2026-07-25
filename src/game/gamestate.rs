use crate::game::board::Board;
use crate::game::pieces::Pieces;

pub struct GameState {
    pub board: Board,
    pub turn: u8,
    pub blue_pieces: Vec<Pieces>,
    pub yellow_pieces: Vec<Pieces>,
    pub red_pieces: Vec<Pieces>,
    pub green_pieces: Vec<Pieces>,
}