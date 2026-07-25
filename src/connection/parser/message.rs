use crate::game::gamestate::GameState;

pub enum MessageType {
    Memento,
    MoveRequest,
    Result,
}

pub struct Message {
    pub message_type: MessageType,
    pub game_state: Option<GameState>,
    pub result: Option<Box<str>>,
}