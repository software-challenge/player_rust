use socha::game::board::Team;
use socha::game::gamerulelogic;
use socha::game::gamestate::GameState;
use socha::client::{Client, start_client_from_commandline_args};
use socha::game::r#move::Move;
use std::time::{SystemTime, UNIX_EPOCH};

struct RandomPlayerClient {
    game_state: Option<socha::game::gamestate::GameState>,
}

impl Client for RandomPlayerClient {
    fn on_move_request(&mut self) -> Option<Move> {
        println!("Received a move request!");

        let legal_moves = gamerulelogic::get_possible_moves(self.game_state.as_ref().unwrap());

        let random_index = random_index(legal_moves.len());
        Some(legal_moves[random_index].clone())
    }

    fn on_game_over(&mut self) {
        println!("Game over!");
    }

    fn on_game_state_updated(&mut self, game_state: GameState ) {
        game_state.board.print_board();
        self.game_state = Some(game_state);
        println!("Game state updated!");
    }
}

fn random_index(max: usize) -> usize {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    (nanos % max as u128) as usize
}

fn main() {
    let client = RandomPlayerClient { game_state: None };
    start_client_from_commandline_args(client).unwrap();
}