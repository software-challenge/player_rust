pub mod gamestate;
pub mod gamerulelogic;
pub mod board;
pub mod piece;
pub mod r#move;
pub mod constants;
pub mod coordinate;
pub mod color;
pub mod rotation;
pub mod parser;
pub mod initialize;

/// The protocol parser of this game, so generic code can name it without
/// knowing the concrete game.
pub use parser::Blokus2027 as Parser;