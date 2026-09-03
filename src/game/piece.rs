use std::{fmt, str::FromStr};

use crate::{game::r#move::Rotation, util::coordinate::{Coordinate, Coordinates}};

pub struct Piece {
    piece_type: PieceType,
    rotation: Rotation,
    is_flipped: bool,
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
        let base_coordinates = self.piece_type.base_coordinates();
        let mut transformed_coordinates: Vec<Coordinate> = base_coordinates.to_vec();

        // Apply rotation
        for coord in &mut transformed_coordinates {
            *coord = coord.rotate(&self.rotation);
        }

        // Apply flipping in board space (left-right mirror)
        if self.is_flipped {
            for coord in &mut transformed_coordinates {
                *coord = coord.flip_on_vertical();
            }
        }

        // Normalize coordinates to start from (0,0)
        transformed_coordinates = Coordinates::normalize_coordinates(&transformed_coordinates);

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

impl PieceType {

    /// Returns all variants of the piece typ
    /// Data Format: Vector<(relative coordinates, (rotation, is flipped?))>
    pub fn all_variants(&self) -> Vec<(Vec<Coordinate>, (Rotation, bool))> {
        let mut variants: Vec<(Vec<Coordinate>, (Rotation, bool))> = Vec::new();
        let base_coordinates = self.base_coordinates();

        // TODO: Filtering out variants that have the same relative coordinates

        for &flip in &[false, true] {
            for &rotation in &[Rotation::None, Rotation::Right, Rotation::Mirror, Rotation::Left] {
                let mut transformed_coordinates: Vec<Coordinate> = base_coordinates.to_vec();

                

                // Apply rotation
                for coord in &mut transformed_coordinates {
                    *coord = coord.rotate(&rotation);
                }

                // Flips from left to right, so we need to flip the coordinates on the vertical axis
                if flip {
                    for coord in &mut transformed_coordinates {
                        *coord = coord.flip_on_vertical();
                    }
                }

                // Normalize coordinates to start from (0,0)
                transformed_coordinates = Coordinates::normalize_coordinates(&transformed_coordinates);

                variants.push((transformed_coordinates, (rotation, flip)))
            }
        }
        variants
    }

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

#[cfg(test)]
mod tests {
    use super::{Piece, PieceType};
    use crate::{game::r#move::Rotation, util::coordinate::Coordinate};

    fn sorted_xy(coordinates: Vec<Coordinate>) -> Vec<(isize, isize)> {
        let mut xy: Vec<(isize, isize)> = coordinates
            .into_iter()
            .map(|coord| (coord.x, coord.y))
            .collect();
        xy.sort_unstable();
        xy
    }

    #[test]
    fn pento_r_get_coordinates_all_8_variants() {
        let cases: Vec<(Rotation, bool, Vec<(isize, isize)>)> = vec![
            (
                Rotation::None,
                false,
                vec![(2, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            ),
            (
                Rotation::Right,
                false,
                vec![(2, 2), (1, 0), (1, 1), (1, 2), (0, 1)],
            ),
            (
                Rotation::Mirror,
                false,
                vec![(0, 2), (2, 1), (1, 1), (0, 1), (1, 0)],
            ),
            (
                Rotation::Left,
                false,
                vec![(0, 0), (1, 2), (1, 1), (1, 0), (2, 1)],
            ),
            (
                Rotation::None,
                true,
                vec![(0, 0), (2, 1), (1, 1), (0, 1), (1, 2)],
            ),
            (
                Rotation::Right,
                true,
                vec![(0, 2), (1, 0), (1, 1), (1, 2), (2, 1)],
            ),
            (
                Rotation::Mirror,
                true,
                vec![(2, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            ),
            (
                Rotation::Left,
                true,
                vec![(2, 0), (1, 2), (1, 1), (1, 0), (0, 1)],
            ),
        ];

        for (rotation, is_flipped, expected) in cases {
            let piece = Piece::new(PieceType::PentoR, rotation, is_flipped);
            let actual = sorted_xy(piece.get_coordinates());

            assert_eq!(
                actual,
                sorted_xy(
                    expected
                        .into_iter()
                        .map(|(x, y)| Coordinate::new(x, y))
                        .collect()
                ),
                "unexpected coordinates for rotation={:?}, is_flipped={}",
                rotation,
                is_flipped
            );
        }
    }
}