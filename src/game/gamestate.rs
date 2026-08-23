use crate::game::board::{Board, Team};
use crate::game::gamestate;
use crate::game::piece::{Piece, PieceType};
use crate::game::r#move::Move;

#[derive(Clone)]
pub struct GameState {
    pub starting_piece: PieceType,
    pub board: Board,
    pub turn: u8,
    pub round: u8,
    pub current_turn_team: Team,
    pieces: [Vec<PieceType>; 4] // blue, yellow, red, green
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
            pieces: [blue_pieces, yellow_pieces, red_pieces, green_pieces],
        }
    }

    /// Applies a move to the game state without any validation.
    /// This function assumes that the move is valid and directly updates the game state.
    /// If the move is invalid, this function may lead to an inconsistent game state.
    pub fn apply_move_unchecked(&mut self, m: &Move, turn: u8) {
        self.board.place_piece(m.x, m.y, m.team, Piece { piece_type: m.piece, is_flipped: m.is_flipped, rotation: m.rotation });
        println!("Applied move: {} {} {} {} {}", m.piece.to_string(), m.x, m.y, m.is_flipped, m.rotation.to_string());

        // Remove used piece from the corresponding team's available pieces
        match m.team {
            Team::Blue => {
                self.pieces[0].retain(|&p| p != m.piece);
            },
            Team::Yellow => {
                self.pieces[1].retain(|&p| p != m.piece);
            },
            Team::Red => {
                self.pieces[2].retain(|&p| p != m.piece);
            },
            Team::Green => {
                self.pieces[3].retain(|&p| p != m.piece);
            },
        }

        self.turn = turn;

        if self.turn % 4 == 0 {
           self.round += 1;
        }


        // TODO: Start team not implemented! Currently using "ONE" as default
        // Current team can be caluclated from turn number
        match self.turn % 4 {
            0 => self.current_turn_team = Team::Blue,
            1 => self.current_turn_team = Team::Yellow,
            2 => self.current_turn_team = Team::Red,
            3 => self.current_turn_team = Team::Green,
            _ => unreachable!(),
        }

        println!("Game state updated: Turn {}, Round {}, Current Team {:?}", self.turn, self.round, self.current_turn_team);
    }

    pub fn get_team_pieces(&self, team: &Team) -> &[PieceType] {
        match team {
            Team::Blue => &self.pieces[0],
            Team::Yellow => &self.pieces[1],
            Team::Red => &self.pieces[2],
            Team::Green => &self.pieces[3],
        }
    }
}