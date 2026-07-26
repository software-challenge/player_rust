use std::{fmt, str::FromStr};

use crate::{game::r#move::Rotation, util::coordinate::Coordinate};

pub struct Piece {
    pub piece_type: PieceType,
    pub rotation: Rotation,
    pub is_flipped: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, rotation: Rotation, is_flipped: bool) -> Self {
        Piece {
            piece_type,
            rotation,
            is_flipped,
        }
    }

    /// Applies the rotation and flipping and returns normalized coordinates from (0,0) in positive direction
    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        todo!();
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PieceType {
    Mono,
    Domino,
    TrioL,
    TrioI,
    TetroO,
    TetroT,
    TetroI,
    TetroL,
    TetroZ,
    PentoL,
    PentoT,
    PentoV,
    PentoS,
    PentoZ,
    PentoI,
    PentoP,
    PentoW,
    PentoU,
    PentoR,
    PentoX,
    PentoY,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PieceType::Mono => "MONO",
            PieceType::Domino => "DOMINO",
            PieceType::TrioL => "TRIO_L",
            PieceType::TrioI => "TRIO_I",
            PieceType::TetroO => "TETRO_O",
            PieceType::TetroT => "TETRO_T",
            PieceType::TetroI => "TETRO_I",
            PieceType::TetroL => "TETRO_L",
            PieceType::TetroZ => "TETRO_Z",
            PieceType::PentoL => "PENTO_L",
            PieceType::PentoT => "PENTO_T",
            PieceType::PentoV => "PENTO_V",
            PieceType::PentoS => "PENTO_S",
            PieceType::PentoZ => "PENTO_Z",
            PieceType::PentoI => "PENTO_I",
            PieceType::PentoP => "PENTO_P",
            PieceType::PentoW => "PENTO_W",
            PieceType::PentoU => "PENTO_U",
            PieceType::PentoR => "PENTO_R",
            PieceType::PentoX => "PENTO_X",
            PieceType::PentoY => "PENTO_Y",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct ParsePieceTypeError;

impl fmt::Display for ParsePieceTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid piece type")
    }
}

impl FromStr for PieceType {
    type Err = ParsePieceTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MONO" => Ok(PieceType::Mono),
            "DOMINO" => Ok(PieceType::Domino),
            "TRIO_L" => Ok(PieceType::TrioL),
            "TRIO_I" => Ok(PieceType::TrioI),
            "TETRO_O" => Ok(PieceType::TetroO),
            "TETRO_T" => Ok(PieceType::TetroT),
            "TETRO_I" => Ok(PieceType::TetroI),
            "TETRO_L" => Ok(PieceType::TetroL),
            "TETRO_Z" => Ok(PieceType::TetroZ),
            "PENTO_L" => Ok(PieceType::PentoL),
            "PENTO_T" => Ok(PieceType::PentoT),
            "PENTO_V" => Ok(PieceType::PentoV),
            "PENTO_S" => Ok(PieceType::PentoS),
            "PENTO_Z" => Ok(PieceType::PentoZ),
            "PENTO_I" => Ok(PieceType::PentoI),
            "PENTO_P" => Ok(PieceType::PentoP),
            "PENTO_W" => Ok(PieceType::PentoW),
            "PENTO_U" => Ok(PieceType::PentoU),
            "PENTO_R" => Ok(PieceType::PentoR),
            "PENTO_X" => Ok(PieceType::PentoX),
            "PENTO_Y" => Ok(PieceType::PentoY),
            _ => Err(ParsePieceTypeError),
        }
    }
}