use crate::game::{board::Team, piece::PieceType};

#[derive(Clone)]
pub struct Move {
    pub team: Team,
    pub piece: PieceType,
    pub x: usize,
    pub y: usize,
    pub is_flipped: bool,
    pub rotation: Rotation,
    pub skip: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Rotation {
    None,
    Right,
    Mirror,
    Left,
}

impl Rotation {
    pub fn from_string(rotation_string: &str) -> Result<Rotation, Box<dyn std::error::Error>> {
        match rotation_string {
            "NONE" => Ok(Rotation::None),
            "RIGHT" => Ok(Rotation::Right),
            "MIRROR" => Ok(Rotation::Mirror),
            "LEFT" => Ok(Rotation::Left),
            _ => Err(format!("Invalid rotation string: {}", rotation_string).into()),
        }
    }

    pub fn from_number(rotation_number: u8) -> Result<Rotation, Box<dyn std::error::Error>> {
        match rotation_number {
            0 => Ok(Rotation::None),
            1 => Ok(Rotation::Right),
            2 => Ok(Rotation::Mirror),
            3 => Ok(Rotation::Left),
            _ => Err(format!("Invalid rotation number: {}", rotation_number).into()),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Rotation::None => "NONE",
            Rotation::Right => "RIGHT",
            Rotation::Mirror => "MIRROR",
            Rotation::Left => "LEFT",
        }
    }
}