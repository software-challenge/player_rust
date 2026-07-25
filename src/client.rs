use crate::connection::handler::ConnectionHandler;
use crate::game::gamestate::GameState;
use crate::connection::parser::message::MessageType;

pub trait Client {
    fn on_move_request(&mut self);
    fn on_game_over(&mut self);
    fn on_game_state_updated(&mut self, gamestate: GameState);
}

pub fn start_client_from_commandline_args<C: Client>(mut client: C) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = ConnectionHandler::new_from_commandline_args()?;

    loop {
        let message = connection.get_new_message()?;

        match message.message_type {
            MessageType::MementoInitial => {
                if let Some(game_state) = message.game_state {
                    client.on_game_state_updated(game_state);
                } else {
                    eprintln!("Received Memento message without game state!");
                }
            },
            MessageType::MementoLastMove => {
                // TODO: Apply the last move to the game state and call on_game_state_updated
            },
            MessageType::MoveRequest => {
                client.on_move_request();
            },
            MessageType::Result => {
                client.on_game_over();
                break;
            },
        }
    }

    Ok(())
}