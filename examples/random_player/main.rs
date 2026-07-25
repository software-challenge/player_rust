use socha::game::gamestate::GameState;
use socha::client::{Client, start_client_from_commandline_args};

struct RandomPlayerClient {
    GameState: Option<socha::game::gamestate::GameState>,
}

impl Client for RandomPlayerClient {
    fn on_move_request(&mut self) {
        // Implement your logic for handling move requests here
    }

    fn on_game_over(&mut self) {
        println!("Game over!");
    }

    fn on_game_state_updated(&mut self, game_state: GameState ) {
        println!("Game state updated!");
    }
}

fn main() {
    let client = RandomPlayerClient { GameState: None };
    start_client_from_commandline_args(client).unwrap();
}