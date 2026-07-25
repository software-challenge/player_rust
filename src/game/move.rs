use crate::game::{board::Team, pieces::Pieces};

pub struct Move {
    pub team: Team,
    pub piece: Pieces,
    pub x: usize,
    pub y: usize,
    // TODO rotation?
    // TODO is fliped?
}
