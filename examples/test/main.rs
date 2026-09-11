use socha::{game::team::Team, prelude::*};

struct Player {
    game_state: Option<GameState>,
}

impl Client for Player {
    fn on_move_request(&mut self) -> Option<Move> {
        println!("Received a move request!");
        get_possible_moves(&self.game_state.as_mut().unwrap()).first().cloned()
    }

    fn on_game_over(&mut self) {
        println!("Game over!");
    }

    fn on_game_state_updated(&mut self, game_state: GameState ) {
        game_state.get_board().print_board();
        self.game_state = Some(game_state);
        
        // Print points of both teams
        let team_one_points = self.game_state.as_ref().unwrap().get_points_for_team(&Team::One);
        let team_two_points = self.game_state.as_ref().unwrap().get_points_for_team(&Team::Two);
        println!("Team One Points: {}", team_one_points);
        println!("Team Two Points: {}", team_two_points);
        
        println!("Game state updated!");
    }
}

fn main() {
    let client = Player { game_state: None };
    start_client_from_commandline_args(client).unwrap();
}