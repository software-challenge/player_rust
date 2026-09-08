use crate::game::{
    color::Color, 
    piece::PieceType, 
    rotation::Rotation
};

#[derive(Clone)]
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
    pub fn new(color: Color, piece: PieceType, x: usize, y: usize, is_flipped: bool, rotation: Rotation, skip: bool) -> Self {
        Self { color, piece, x, y, is_flipped, rotation, skip }
    }
}

