use crate::{
    common::team::Team,
    games::blokus2027::color::Color::{self, Blue, Yellow, Red, Green},
};

/// The mapping of teams to colors is game specific and therefore lives in
/// the game module rather than in `common::team`.
impl Team {
    pub fn get_team_colors(&self) -> [Color; 2] {
        match self {
            Team::One => [Blue, Red],
            Team::Two => [Yellow, Green],
        }
    }
}
