use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Rotation {
    None,
    Right,
    Mirror,
    Left,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::None => write!(f, "NONE"),
            Rotation::Right => write!(f, "RIGHT"),
            Rotation::Mirror => write!(f, "MIRROR"),
            Rotation::Left => write!(f, "LEFT"),
        }
    }
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
}