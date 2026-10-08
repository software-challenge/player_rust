/// One of the four player colors in the game.
///
/// The color names correspond to the protocol values used by the server and the
/// `GameState` data model: `BLUE`, `YELLOW`, `RED`, and `GREEN`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    Blue,
    Yellow,
    Red,
    Green,
}

impl TryFrom<&str> for Color {
    type Error = String;

    /// Parses a color string in uppercase. The valid options are: `"BLUE"`, `"YELLOW"`, `"RED"`, `"GREEN"`.
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
    /// Formats the color as a string in uppercase, matching the protocol values.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Blue => write!(f, "BLUE"),
            Color::Yellow => write!(f, "YELLOW"),
            Color::Red => write!(f, "RED"),
            Color::Green => write!(f, "GREEN"),
        }
    }
}