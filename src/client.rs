use crate::connection::handler::ConnectionHandler;
use crate::game::gamestate::GameState;
use crate::connection::parser::message::MessageType;
use crate::game::r#move::Move;

pub trait Client {
    fn on_move_request(&mut self) -> Option<Move>;
    fn on_game_over(&mut self);
    fn on_game_state_updated(&mut self, gamestate: GameState);
}

pub fn start_client_from_commandline_args<C: Client>(mut client: C) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = ConnectionHandler::new_from_commandline_args()?;

    let mut local_game_state: Option<GameState> = None;

    loop {
        let message = connection.get_new_message()?;

        match message.message_type {
            MessageType::MementoInitial => {
                if let Some(game_state) = message.game_state {
                    local_game_state = Some(game_state);
                    client.on_game_state_updated(local_game_state.as_ref().unwrap().clone());
                } else {
                    eprintln!("Received Memento message without game state!");
                }
            },
            MessageType::MementoLastMove => {
                if let Some(last_move) = message.last_move {
                    if let Some(game_state) = &mut local_game_state {
                        println!("Applying last move: {} {} {} {} {}", last_move.piece, last_move.x, last_move.y, last_move.is_flipped, last_move.rotation.to_string());
                        game_state.apply_move_unchecked(&last_move, message.turn.unwrap_or(0));
                        client.on_game_state_updated(game_state.clone());
                    } else {
                        eprintln!("Received LastMove message without existing game state!");
                    }
                } else {
                    eprintln!("Received LastMove message without last move data!");
                }
            },
            MessageType::MoveRequest => {
                let m = client.on_move_request();
                if let Some(mv) = m {
                    connection.send_move(&mv)?;
                } else {
                    eprintln!("Client did not provide a move in response to MoveRequest!");
                }
            },
            MessageType::Result => {
                client.on_game_over();
                break;
            },
        }
    }

    Ok(())
}