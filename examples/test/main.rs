use socha::game::{gamestate::GameState, gamerulelogic, r#move::Move};
use socha::client::{Client, start_client_from_commandline_args};

struct Player {
    game_state: Option<socha::game::gamestate::GameState>,
}

impl Client for Player {
    fn on_move_request(&mut self) -> Option<Move> {
        println!("Received a move request!");
        gamerulelogic::get_possible_moves(&self.game_state.as_mut().unwrap()).first().cloned()
    }

    fn on_game_over(&mut self) {
        println!("Game over!");
    }

    fn on_game_state_updated(&mut self, game_state: GameState ) {
        game_state.get_board().print_board();
        self.game_state = Some(game_state);
        println!("Game state updated!");
    }
}

fn main() {
    let client = Player { game_state: None };
    start_client_from_commandline_args(client).unwrap();
}