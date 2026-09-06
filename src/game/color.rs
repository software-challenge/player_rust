#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub enum Color {
    Blue,
    Yellow,
    Red,
    Green,
}

impl Color {
    pub fn from_string(s: &str) -> Self {
        match s {
            "BLUE" => Color::Blue,
            "YELLOW" => Color::Yellow,
            "RED" => Color::Red,
            "GREEN" => Color::Green,
            _ => panic!("Unknown color: {}", s),
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