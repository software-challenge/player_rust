use crate::{
    connection::parser::parse_result::GameResult, 
    game::{
        gamestate::GameState, 
        r#move::Move
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    MementoInitial(Option<GameState>),
    MementoLastMove(Option<u8>, Option<Move>),
    MoveRequest,
    Result(Option<GameResult>),
}