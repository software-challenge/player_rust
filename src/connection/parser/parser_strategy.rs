use crate::games::active::{gamestate::GameState, r#move::Move};

/// The result of parsing a memento.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MementoReturn {
    /// The full game state.
    GameState(Box<GameState>),
    /// The last move, with the turn.
    LastMove(u8, Move)
}

/// Strategy for parsing memento XML into a [`MementoReturn`].
/// This needs to be implemented for the game specific memento parser.
pub trait ParserStrategy {
/// Parses the game specific memento.
/// Should return a [`MementoReturn::GameState`] on the first memento received,
/// and [`MementoReturn::LastMove`] on every one after that.
    fn parse_memento(xml: &str) -> Result<MementoReturn, Box<dyn std::error::Error>>;
}