use crate::game::{
    color::Color, 
    piece::PieceType, 
    rotation::Rotation
};

/// A single move requested from or sent to the game server.
///
/// A move either places a piece on the board or represents a skip/pass move.
/// The placement origin is the top-left corner of the piece's transformed local
/// coordinate system.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Move {
    pub color: Color,
    pub piece: PieceType,
    pub x: usize,
    pub y: usize,
    pub is_flipped: bool,
    pub rotation: Rotation,
    pub skip: bool,
}

impl Move {
    /// Creates a new move instance.
    pub fn new(color: Color, piece: PieceType, x: usize, y: usize, is_flipped: bool, rotation: Rotation, skip: bool) -> Self {
        Self { color, piece, x, y, is_flipped, rotation, skip }
    }
}

