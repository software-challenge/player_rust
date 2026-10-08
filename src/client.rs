use crate::{connection::{
                handler::ConnectionHandler, 
                parser::message::Message
            }, game::{
                gamestate::GameState, r#move::Move, parser::Blokus2026
            }
        };

/// A player implementation that handles the game flow.
///
/// The trait is driven by the server protocol: a new `GameState` is pushed via
/// `on_game_state_updated`, a move is requested through `on_move_request`, and
/// the game ends with `on_game_over`.
pub trait Client {
    /// Called when the server asks the player to submit a move.
    fn on_move_request(&mut self) -> Option<Move>;

    /// Called after the final result message is received.
    fn on_game_over(&mut self);

    /// Called whenever the engine updates the current board state.
    fn on_game_state_updated(&mut self, gamestate: GameState);
}

/// Starts a client instance using the command line arguments passed to the program.
///
/// This helper configures the network connection and dispatches server messages
/// to the `Client` trait methods until the match ends.
pub fn start_client_from_commandline_args<C: Client>(mut client: C) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = ConnectionHandler::new_from_commandline_args(Blokus2026)?;

    let mut local_game_state: Option<GameState> = None;

    loop {
        let message = connection.get_new_message()?;
        match *message {
            Message::MementoInitial(game_state) => {
                if let Some(game_state) = game_state {
                    local_game_state = Some(game_state.clone());
                    client.on_game_state_updated(game_state);
                } else {
                    eprintln!("Received Memento message without game state!");
                }
            },
            Message::MementoLastMove(turn, last_move) => {
                if let Some(last_move) = last_move {
                    if let Some(game_state) = &mut local_game_state {
                        #[cfg(feature = "debug-print")]
                        println!("Applying last move: {} {} {} {} {}", last_move.piece, last_move.x, last_move.y, last_move.is_flipped, last_move.rotation);
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