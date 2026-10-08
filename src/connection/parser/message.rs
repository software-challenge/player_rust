use crate::{
    connection::parser::parse_result::GameResult, 
    games::active::{
        gamestate::GameState, 
        r#move::Move
    }
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    MementoInitial(Option<GameState>),
    MementoLastMove(Option<u8>, Option<Move>),
    MoveRequest,
    Result(Option<GameResult>),
}