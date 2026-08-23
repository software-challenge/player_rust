use crate::game::{gamestate::GameState, r#move::Move};

pub enum MessageType {
    MementoInitial,
    MementoLastMove,
    MoveRequest,
    Result,
}

pub struct Message {
    pub message_type: MessageType,
    pub game_state: Option<GameState>,
    pub last_move: Option<Box<Move>>,
    pub turn: Option<u8>,
    pub result: Option<Box<str>>,
}