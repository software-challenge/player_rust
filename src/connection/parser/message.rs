use crate::{
    connection::parser::parse_result::GameResult, 
    games::active::{
        gamestate::GameState, 
        r#move::Move
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    MementoInitial(Box<GameState>),
    MementoLastMove(u8, Move),
    MoveRequest,
    Result(GameResult),
}