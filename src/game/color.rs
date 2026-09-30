#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    Blue,
    Yellow,
    Red,
    Green,
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "BLUE" => Ok(Color::Blue),
            "YELLOW" => Ok(Color::Yellow),
            "RED" => Ok(Color::Red),
            "GREEN" => Ok(Color::Green),
            _ => Err(format!("Unknown color: {s}")),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Blue => write!(f, "BLUE"),
            Color::Yellow => write!(f, "YELLOW"),
            Color::Red => write!(f, "RED"),
            Color::Green => write!(f, "GREEN"),
        }
    }
}