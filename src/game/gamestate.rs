use crate::game::board::{Board, Team};
use crate::game::piece::{Piece, PieceType};
use crate::game::r#move::Move;

#[derive(Clone)]
pub struct GameState {
    pub starting_piece: PieceType,
    pub board: Board,
    pub turn: u8,
    pub round: u8,
    pub current_turn_team: Team,
    blue_pieces: Vec<PieceType>,
    yellow_pieces: Vec<PieceType>,
    red_pieces: Vec<PieceType>,
    green_pieces: Vec<PieceType>,
}

// TODO: Replace color pieces with a HashMap<Team, Vec<PieceType>> to make it more flexible and easier to manage.

impl GameState {

    pub fn new(starting_piece: PieceType, board: Board, turn: u8, round: u8, current_turn_team: Team, blue_pieces: Vec<PieceType>, yellow_pieces: Vec<PieceType>, red_pieces: Vec<PieceType>, green_pieces: Vec<PieceType>) -> Self {
        GameState {
            starting_piece,
            board,
            turn,
            round,
            current_turn_team,
            blue_pieces,
            yellow_pieces,
            red_pieces,
            green_pieces,
        }
    }

    /// Applies a move to the game state without any validation.
    /// This function assumes that the move is valid and directly updates the game state.
    /// If the move is invalid, this function may lead to an inconsistent game state.
    pub fn apply_move_unchecked(&mut self, m: &Move) {
        self.board.place_piece(m.x, m.y, m.team, Piece { piece_type: m.piece, is_flipped: m.is_flipped, rotation: m.rotation });
        println!("Applied move: {} {} {} {} {}", m.piece.to_string(), m.x, m.y, m.is_flipped, m.rotation.to_string());

        // TODO: Skip moves are not handled yet
        match m.team {
            Team::Blue => {
                self.blue_pieces.retain(|&p| p != m.piece);
                self.current_turn_team = Team::Yellow;
            },
            Team::Yellow => {
                self.yellow_pieces.retain(|&p| p != m.piece);
                self.current_turn_team = Team::Red;
            },
            Team::Red => {
                self.red_pieces.retain(|&p| p != m.piece);
                self.current_turn_team = Team::Green;
            },
            Team::Green => {
                self.green_pieces.retain(|&p| p != m.piece);
                self.current_turn_team = Team::Blue;
            },
        }

        self.turn += 1;
    }

    pub fn get_team_pieces(&self, team: &Team) -> &[PieceType] {
        match team {
            Team::Blue => &self.blue_pieces,
            Team::Yellow => &self.yellow_pieces,
            Team::Red => &self.red_pieces,
            Team::Green => &self.green_pieces,
        }
    }
}