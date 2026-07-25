use crate::connection::handler::ConnectionHandler;
use crate::game::gamestate::GameState;

pub trait Client {
    fn on_move_request(&mut self);
    fn on_game_over(&mut self);
    fn on_game_state_updated(&mut self, gamestate: GameState);
}

pub fn start_client_from_commandline_args<C: Client>(client: C) -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = ConnectionHandler::new_from_commandline_args()?;

    loop {
        connection.get_new_message()?;
    }

    Ok(())
}