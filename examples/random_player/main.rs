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

        // Is not true after the first skip
        let mut current_team = Team::Yellow;
        match self.game_state.as_ref().unwrap().turn % 4 {
            0 => current_team = Team::Blue,
            1 => current_team = Team::Yellow,
            2 => current_team = Team::Red,
            3 => current_team = Team::Green,
            _ => {}
        }

        if self.game_state.as_ref().unwrap().turn > 4 {
            return Some(Move {
                team: current_team,
                piece: self.game_state.as_ref().unwrap().starting_piece,
                x: 0,
                y: 0,
                is_flipped: false,
                rotation: socha::game::r#move::Rotation::None,
                skip: true
            })
        }

        let legal_moves = gamerulelogic::get_legal_moves_for_team(self.game_state.as_ref().unwrap(), &current_team);

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