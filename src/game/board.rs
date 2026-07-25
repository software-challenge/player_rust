pub struct Board {
    pub board: [[Option<Team>; 20]; 20],
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None; 20]; 20],
        }
    }
}

#[derive(Copy, Clone)]
pub enum Team {
    Blue,
    Yellow,
    Red,
    Green,
}