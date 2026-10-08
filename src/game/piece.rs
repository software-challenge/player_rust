#[cfg(test)]
#[path = "tests/piece.rs"]
mod tests;

use std::{fmt, str::FromStr};

use crate::game::{
    coordinate::{
        Coordinate, 
        flip_coordinates, 
        rotate_coordinates, 
        normalize_coordinates
    },
    rotation::Rotation,
};

/// A concrete piece instance with a type, a rotation, and a mirror state.
///
/// The piece stores its local coordinates relative to the top-left origin and can
/// be converted into board-relative coordinates through [`Piece::get_coordinates`].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Piece {
    piece_type: PieceType,
    rotation: Rotation,
    is_flipped: bool,
}

impl Piece {
    /// Creates a piece from its type, rotation, and flip state.
    pub fn new(piece_type: PieceType, rotation: Rotation, is_flipped: bool) -> Self {
        Piece {
            piece_type,
            rotation,
            is_flipped,
        }
    }

    /// Returns the transformed coordinates of the piece, normalized to the origin.
    ///
    /// The coordinates are rotated and optionally mirrored and then shifted so the
    /// smallest x/y values become `(0, 0)`.
    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        let base_coordinates = self.piece_type.base_coordinates();
        let mut transformed_coordinates: Vec<Coordinate> = base_coordinates.to_vec();

        rotate_coordinates(&mut transformed_coordinates, &self.rotation);
        if self.is_flipped { 
            flip_coordinates(&mut transformed_coordinates);
        }
        normalize_coordinates(&mut transformed_coordinates);

        transformed_coordinates
    }

    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }

    pub fn set_piece_type(&mut self, piece_type: PieceType) {
        self.piece_type = piece_type;
    }

    pub fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    pub fn is_flipped(&self) -> &bool {
        &self.is_flipped
    }

    pub fn set_flipped(&mut self, is_flipped: bool) {
        self.is_flipped = is_flipped;
    }
}

/// The set of available Blokus piece shapes.
///
/// Each variant describes a shape and can be transformed into all legal
/// rotations and mirrored states using [`PieceType::all_variants`].
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

pub const ALL_PIECE_TYPES: [PieceType; 21] = [
    PieceType::Mono,
    PieceType::Domino,
    PieceType::TrioL,
    PieceType::TrioI,
    PieceType::TetroO,
    PieceType::TetroT,
    PieceType::TetroI,
    PieceType::TetroL,
    PieceType::TetroZ,
    PieceType::PentoL,
    PieceType::PentoT,
    PieceType::PentoV,
    PieceType::PentoS,
    PieceType::PentoZ,
    PieceType::PentoI,
    PieceType::PentoP,
    PieceType::PentoW,
    PieceType::PentoU,
    PieceType::PentoR,
    PieceType::PentoX,
    PieceType::PentoY,
];

impl PieceType {

    /// Returns all orientation variants of the piece type.
    ///
    /// Each entry contains the normalized coordinates for the shape and the
    /// resulting `(rotation, is_flipped)` pair. When `filter` is `true`,
    /// geometrically identical orientations are deduplicated.
    pub fn all_variants(&self, filter: bool) -> Vec<(Vec<Coordinate>, (Rotation, bool))> {
        let mut variants: Vec<(Vec<Coordinate>, (Rotation, bool))> = Vec::new();
        let base_coordinates = self.base_coordinates();

        for &flip in &[false, true] {
            for &rotation in &[Rotation::None, Rotation::Right, Rotation::Mirror, Rotation::Left] {
                let mut transformed_coordinates: Vec<Coordinate> = base_coordinates.to_vec();

                rotate_coordinates(&mut transformed_coordinates, &rotation);
                if flip { 
                    flip_coordinates(&mut transformed_coordinates);
                }
                normalize_coordinates(&mut transformed_coordinates);

                if filter {
                    // Check if the transformed coordinates already exist in the variants vector
                    if variants.iter().any(|(coords, _)| *coords == transformed_coordinates) {
                        continue; // Skip adding this variant as it already exists
                    }
                }   

                variants.push((transformed_coordinates, (rotation, flip)))
            }
        }
        variants
    }

    /// Returns the untransformed local coordinates of the piece type.
    ///
    /// These base coordinates start at `(0, 0)` and grow only toward positive x/y
    /// values; they do not include any rotation or flipping.
    pub fn base_coordinates(&self) -> &'static [Coordinate] {
        match self {
            PieceType::Mono => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0)];
                COORDS
            }
            PieceType::Domino => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(1, 0)];
                COORDS
            }
            PieceType::TrioL => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(1, 1)];
                COORDS
            }
            PieceType::TrioI => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2)];
                COORDS
            }
            PieceType::TetroO => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1)];
                COORDS
            }
            PieceType::TetroT => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(2, 0), Coordinate::new(1, 1)];
                COORDS
            }
            PieceType::TetroI => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(0, 3)];
                COORDS
            }
            PieceType::TetroL => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(1, 2)];
                COORDS
            }
            PieceType::TetroZ => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, 1), Coordinate::new(2, 1)];
                COORDS
            }
            PieceType::PentoL => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(0, 3), Coordinate::new(1, 3)];
                COORDS
            }
            PieceType::PentoT => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(2, 0), Coordinate::new(1, 1), Coordinate::new(1, 2)];
                COORDS
            }
            PieceType::PentoV => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(1, 2), Coordinate::new(2, 2)];
                COORDS
            }
            PieceType::PentoS => {
                const COORDS: &[Coordinate] = &[Coordinate::new(1, 0), Coordinate::new(2, 0), Coordinate::new(3, 0), Coordinate::new(0, 1), Coordinate::new(1, 1)];
                COORDS
            }
            PieceType::PentoZ => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, 1), Coordinate::new(1, 2), Coordinate::new(2, 2)];
                COORDS
            }
            PieceType::PentoI => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(0, 3), Coordinate::new(0, 4)];
                COORDS
            }
            PieceType::PentoP => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(0, 2)];
                COORDS
            }
            PieceType::PentoW => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(1, 2), Coordinate::new(2, 2)];
                COORDS
            }
            PieceType::PentoU => {
                const COORDS: &[Coordinate] = &[Coordinate::new(0, 0), Coordinate::new(2, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(2, 1)];
                COORDS
            }
            PieceType::PentoR => {
                const COORDS: &[Coordinate] = &[Coordinate::new(2, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(2, 1), Coordinate::new(1, 2)];
                COORDS
            }
            PieceType::PentoX => {
                const COORDS: &[Coordinate] = &[Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(2, 1), Coordinate::new(1, 2)];
                COORDS
            }
            PieceType::PentoY => {
                const COORDS: &[Coordinate] = &[Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(1, 2), Coordinate::new(1, 3)];
                COORDS
            }
        }
    }
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

/// Returned when a text value does not match a valid piece type name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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