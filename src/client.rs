use crate::connection::handler::ConnectionHandler;
use crate::game::gamestate::GameState;
use crate::connection::parser::message::Message;
use crate::game::r#move::Move;

pub trait Client {
    fn on_move_request(&mut self) -> Option<Move>;
    fn on_game_over(&mut self);
    fn on_game_state_updated(&mut self, gamestate: GameState);
}

/// Starts a new client using the commandline args to connect to the game.
pub fn start_client_from_commandline_args<C: Client>(mut client: C) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = ConnectionHandler::new_from_commandline_args()?;

    let mut local_game_state: Option<GameState> = None;

    loop {
        let message = connection.get_new_message()?;
        println!("size of {}", size_of::<Message>());
        match *message {
            Message::MementoInitial(game_state) => {
                if let Some(game_state) = game_state {
                    local_game_state = Some(game_state);
                    client.on_game_state_updated(local_game_state.as_ref().unwrap().clone());
                } else {
                    eprintln!("Received Memento message without game state!");
                }
            },
            Message::MementoLastMove(turn, last_move) => {
                if let Some(last_move) = last_move {
                    if let Some(game_state) = &mut local_game_state {
                        println!("Applying last move: {} {} {} {} {}", last_move.piece, last_move.x, last_move.y, last_move.is_flipped, last_move.rotation.to_string());
                        game_state.apply_move_unchecked(&last_move, turn.unwrap_or(0));
                        client.on_game_state_updated(game_state.clone());
                    } else {
                        eprintln!("Received LastMove message without existing game state!");
                    }
                } else {
                    eprintln!("Received LastMove message without last move data!");
                }
            },
            Message::MoveRequest => {
                let m = client.on_move_request();
                if let Some(mv) = m {
                    connection.send_move(&mv)?;
                } else {
                    eprintln!("Client did not provide a move in response to MoveRequest!");
                }
            },
            Message::Result(_) => {
                client.on_game_over();
                break;
            },
        }
    }

    Ok(())
}