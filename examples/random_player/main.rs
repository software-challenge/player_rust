use socha::game::gamestate::GameState;
use socha::client::{Client, start_client_from_commandline_args};

struct RandomPlayerClient {
    game_state: Option<socha::game::gamestate::GameState>,
}

impl Client for RandomPlayerClient {
    fn on_move_request(&mut self) {
        // Implement your logic for handling move requests here
        println!("Received a move request!");
    }

    fn on_game_over(&mut self) {
        println!("Game over!");
    }

    fn on_game_state_updated(&mut self, game_state: GameState ) {
        self.game_state = Some(game_state);

        for piece in &self.game_state.as_ref().unwrap().blue_pieces {
            println!("Blue piece: {}", piece.to_string());
        }

        self.game_state.as_ref().unwrap().board.print_board();

        println!("Game state updated!");
    }
}

fn main() {
    let client = RandomPlayerClient { game_state: None };
    start_client_from_commandline_args(client).unwrap();
}