#[cfg(test)]
#[path = "tests/piece.rs"]
mod tests;

use std::{fmt, str::FromStr};

use crate::games::blokus2027::{coordinate::Coordinate, rotation::Rotation};

#[cfg(test)]
use crate::games::blokus2027::coordinate::{
    flip_coordinates, normalize_coordinates, rotate_coordinates,
};

const ROTATION_NUM: usize = 4; // Amount of different rotations
const MAX_VARIANT_COORDS: usize = 5; // Maximum amount of coordinates a piece has
const PIECE_VARIANT_COUNT: usize = 2 * ROTATION_NUM * ALL_PIECE_TYPES.len();

/// The untransformed base coordinates of each piece type (indexed by
/// `PieceType as usize`), padded with dummy coordinates to
/// `MAX_VARIANT_COORDS`.
///
/// This is the single source of the piece shapes: both the compile-time variant
/// table and [`PieceType::base_coordinates`] are derived from it.
static PIECE_BASES: [([Coordinate; MAX_VARIANT_COORDS], usize); ALL_PIECE_TYPES.len()] = build_piece_bases();

/// All `PIECE_VARIANT_COUNT` piece variants (2 flips × 4 rotations × 21 piece
/// types), normalized to the origin and computed at compile time.
///
/// Each entry holds the coordinates (padded to `MAX_VARIANT_COORDS`) and their
/// actual length. The index follows the order of `piece_orientation_to_index`.
static PIECE_VARIANTS: [([Coordinate; MAX_VARIANT_COORDS], usize); PIECE_VARIANT_COUNT] = build_piece_variants();

#[allow(clippy::indexing_slicing)]
const fn build_piece_variants() -> [([Coordinate; MAX_VARIANT_COORDS], usize); PIECE_VARIANT_COUNT] {
    const FLIPS: [bool; 2] = [false, true];
    const ROTATIONS: [Rotation; ROTATION_NUM] = [Rotation::None, Rotation::Right, Rotation::Mirror, Rotation::Left];

    let mut variants = [([Coordinate::new(0, 0); MAX_VARIANT_COORDS], 0usize); PIECE_VARIANT_COUNT];

    // Important: Do not change the order of is_flipped, rotation, piece_type.
    // This must match the enum discriminant ordering used by PieceType as usize.
    // (While loops: `for` over `Range` is not const-stable.)
    let mut flip_idx = 0;
    while flip_idx < 2 {
        let mut rotation_idx = 0;
        while rotation_idx < ROTATIONS.len() {
            let mut piece_idx = 0;
            while piece_idx < ALL_PIECE_TYPES.len() {
                let (mut coords, len) = PIECE_BASES[piece_idx];
                let mut i = 0;

                while i < len {
                    coords[i] = coords[i].rotate(&ROTATIONS[rotation_idx]);
                    i += 1;
                }

                if FLIPS[flip_idx] {
                    let mut i = 0;
                    while i < len {
                        coords[i] = coords[i].flip_on_vertical();
                        i += 1;
                    }
                }

                let index = ((flip_idx * ROTATION_NUM) + rotation_idx) * ALL_PIECE_TYPES.len() + piece_idx;
                variants[index] = (normalize_variant(coords, len), len);
                piece_idx += 1;
            }
            rotation_idx += 1;
        }
        flip_idx += 1;
    }

    variants
}

/// Shifts the coordinates so the smallest x/y values become `(0, 0)`.
#[allow(clippy::indexing_slicing)]
const fn normalize_variant(mut coords: [Coordinate; MAX_VARIANT_COORDS], len: usize) -> [Coordinate; MAX_VARIANT_COORDS] {
    let (mut min_x, mut min_y) = (coords[0].x, coords[0].y);
    let mut i = 1;
    while i < len {
        // Plain comparison: `Ord::min` is not const-stable.
        if coords[i].x < min_x { min_x = coords[i].x; }
        if coords[i].y < min_y { min_y = coords[i].y; }
        i += 1;
    }
    let mut i = 0;
    while i < len {
        coords[i].x -= min_x;
        coords[i].y -= min_y;
        i += 1;
    }
    coords
}

/// Generates all piece variants at runtime.
///
/// Kept for tests only: it verifies that the compile-time `PIECE_VARIANTS`
/// table matches the runtime transformations entry by entry.
#[cfg(test)]
fn generate_piece_variants() -> Vec<Vec<Coordinate>> {
    let mut variants: Vec<Vec<Coordinate>> = Vec::with_capacity(PIECE_VARIANT_COUNT);

    // Important: Do not change the order of is_flipped, rotation, piece_type.
    // This must match the enum discriminant ordering used by PieceType as usize.
    for is_flipped in [false, true] {
        for rotation in [Rotation::None, Rotation::Right, Rotation::Mirror, Rotation::Left] {
            for piece_type in ALL_PIECE_TYPES {
                let base_coordinates = piece_type.base_coordinates();
                let mut transformed_coordinates: Vec<Coordinate> = base_coordinates.to_vec();

                rotate_coordinates(&mut transformed_coordinates, &rotation);
                if is_flipped {
                    flip_coordinates(&mut transformed_coordinates);
                }
                normalize_coordinates(&mut transformed_coordinates);

                variants.push(transformed_coordinates);
            }
        }
    }

    variants
}

// Maps piece variant to the corresponding memory position.
// The piece index must follow the generation order: flip, rotation, then piece type.
fn piece_orientation_to_index(is_flipped: bool, rotation: Rotation, piece_type: PieceType) -> usize {
    ((is_flipped as usize) * ROTATION_NUM + (rotation as usize)) * ALL_PIECE_TYPES.len() + (piece_type as usize)
}

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
    #[allow(clippy::indexing_slicing)] // Indexing is safe: the index is derived from
    // `piece_orientation_to_index` and always within the table.
    pub fn get_coordinates(&self) -> Vec<Coordinate> {
        let index = piece_orientation_to_index(*self.is_flipped(), self.rotation, self.piece_type);
        let (coords, len) = PIECE_VARIANTS[index];
        coords[..len].to_vec()
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
#[repr(usize)]
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

/// A array containing all piece types
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

const fn build_piece_bases() -> [([Coordinate; MAX_VARIANT_COORDS], usize); ALL_PIECE_TYPES.len()] {
    [
        // Mono
        ([Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0)], 1),
        // Domino
        ([Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(0, 0), Coordinate::new(0, 0), Coordinate::new(0, 0)], 2),
        // TrioL
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(0, 0), Coordinate::new(0, 0)], 3),
        // TrioI
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(0, 0), Coordinate::new(0, 0)], 3),
        // TetroO
        ([Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(0, 0)], 4),
        // TetroT
        ([Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(2, 0), Coordinate::new(1, 1), Coordinate::new(0, 0)], 4),
        // TetroI
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(0, 3), Coordinate::new(0, 0)], 4),
        // TetroL
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(1, 2), Coordinate::new(0, 0)], 4),
        // TetroZ
        ([Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, 1), Coordinate::new(2, 1), Coordinate::new(0, 0)], 4),
        // PentoL
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(0, 3), Coordinate::new(1, 3)], 5),
        // PentoT
        ([Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(2, 0), Coordinate::new(1, 1), Coordinate::new(1, 2)], 5),
        // PentoV
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(1, 2), Coordinate::new(2, 2)], 5),
        // PentoS
        ([Coordinate::new(1, 0), Coordinate::new(2, 0), Coordinate::new(3, 0), Coordinate::new(0, 1), Coordinate::new(1, 1)], 5),
        // PentoZ
        ([Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(1, 1), Coordinate::new(1, 2), Coordinate::new(2, 2)], 5),
        // PentoI
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(0, 2), Coordinate::new(0, 3), Coordinate::new(0, 4)], 5),
        // PentoP
        ([Coordinate::new(0, 0), Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(0, 2)], 5),
        // PentoW
        ([Coordinate::new(0, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(1, 2), Coordinate::new(2, 2)], 5),
        // PentoU
        ([Coordinate::new(0, 0), Coordinate::new(2, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(2, 1)], 5),
        // PentoR
        ([Coordinate::new(2, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(2, 1), Coordinate::new(1, 2)], 5),
        // PentoX
        ([Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(2, 1), Coordinate::new(1, 2)], 5),
        // PentoY
        ([Coordinate::new(1, 0), Coordinate::new(0, 1), Coordinate::new(1, 1), Coordinate::new(1, 2), Coordinate::new(1, 3)], 5),
    ]
}

impl PieceType {

    /// Returns all orientation variants of the piece type.
    ///
    /// Each entry contains the normalized coordinates for the shape and the
    /// resulting `(rotation, is_flipped)` pair. When `filter` is `true`,
    /// geometrically identical orientations are deduplicated.
    pub fn all_variants(&self, filter: bool) -> Vec<(Vec<Coordinate>, (Rotation, bool))> {
        let mut variants: Vec<(Vec<Coordinate>, (Rotation, bool))> =
            Vec::with_capacity(2 * ROTATION_NUM);

        for &flip in &[false, true] {
            for &rotation in &[
                Rotation::None,
                Rotation::Right,
                Rotation::Mirror,
                Rotation::Left,
            ] {
                let index = piece_orientation_to_index(flip, rotation, *self);
                let (coordinates, len) = PIECE_VARIANTS[index];
                let transformed_coordinates = coordinates[..len].to_vec();

                if filter {
                    // Check if the transformed coordinates already exist in the variants vector
                    if variants.iter().any(|(coords, _)| *coords == transformed_coordinates) {
                        continue; // Skip adding this variant as it already exists
                    }
                }   

                variants.push((transformed_coordinates, (rotation, flip)));
            }
        }
        variants
    }

    /// Returns the untransformed local coordinates of the piece type.
    ///
    /// These base coordinates start at `(0, 0)` and grow only toward positive x/y
    /// values; they do not include any rotation or flipping.
    #[allow(clippy::indexing_slicing)]
    pub fn base_coordinates(&self) -> &'static [Coordinate] {
        let len = PIECE_BASES[*self as usize].1;
        &PIECE_BASES[*self as usize].0[..len]
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