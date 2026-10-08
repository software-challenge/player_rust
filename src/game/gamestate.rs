#[cfg(test)]
#[path = "tests/gamestate.rs"]
mod tests;

use crate::game::{
    board::Board,
    color::Color,
    gamerulelogic,
    r#move::Move,
    piece::{
        Piece, 
        PieceType
    }
};
/// Full snapshot of the current game state.
///
/// In addition to the board and the active player, the state also keeps track of
/// each team's remaining pieces, the current turn/round, and per-color scoring
/// information.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameState {
    starting_piece: PieceType,
    is_starting_team_one: bool,
    board: Board,
    turn: u8,
    round: u8,
    current_turn_color: Color,
    points: [u8; 4], // points for each color; blue, yellow, red, green
    last_move: [Option<Move>; 4], // last move of each color; blue, yellow, red, green
    pieces: [Vec<PieceType>; 4] // blue, yellow, red, green
}

impl GameState {

    /// Creates a new game state from the supplied board and piece state.
    #[allow(clippy::too_many_arguments)]
    pub fn new(starting_piece: PieceType, is_starting_team_one: bool, board: Board, turn: u8, round: u8, current_turn_color: Color, blue_pieces: Vec<PieceType>, yellow_pieces: Vec<PieceType>, red_pieces: Vec<PieceType>, green_pieces: Vec<PieceType>) -> Self {
        GameState {
            starting_piece,
            is_starting_team_one,
            board,
            turn,
            round,
            current_turn_color,
            points: [0, 0, 0, 0],
            last_move: [None, None, None, None],
            pieces: [blue_pieces, yellow_pieces, red_pieces, green_pieces],
        }
    }

    /// Applies a move without validating it.
    ///
    /// Use this only when the move is already known to be legal. It updates the
    /// board and the derived state fields directly.
    pub fn apply_move_unchecked(&mut self, m: &Move, turn: u8) {
        self.board.place_piece_unchecked(m.x, m.y, m.color, Piece::new(m.piece, m.rotation, m.is_flipped));
        self.update_gamestate(m, turn);
    }

    /// Applies a move after validating it against the current game state.
    ///
    /// Returns `true` if the move was accepted and applied, or `false` if it was
    /// illegal in the current position.
    pub fn apply_move(&mut self, m: &Move, turn: u8) -> bool {
        if !gamerulelogic::is_valid_move(self, m) {
            return false;
        }

        self.board.place_piece(m.x, m.y, m.color, Piece::new(m.piece, m.rotation, m.is_flipped));
        self.update_gamestate(m, turn);

        true
    }

    /// Updates the game state after a move has been applied to the board either save or unsave.
    fn update_gamestate(&mut self, m: &Move, turn: u8) {
        // Remove used piece from the corresponding color's available pieces
        match m.color {
            Color::Blue => {
                self.pieces[0].retain(|&p| p != m.piece);
                self.last_move[0] = Some(*m);
                self.points[0] = self.calculate_points_for_color(&Color::Blue);
            },
            Color::Yellow => {
                self.pieces[1].retain(|&p| p != m.piece);
                self.last_move[1] = Some(*m);
                self.points[1] = self.calculate_points_for_color(&Color::Yellow);
            },
            Color::Red => {
                self.pieces[2].retain(|&p| p != m.piece);
                self.last_move[2] = Some(*m);
                self.points[2] = self.calculate_points_for_color(&Color::Red);
            },
            Color::Green => {
                self.pieces[3].retain(|&p| p != m.piece);
                self.last_move[3] = Some(*m);
                self.points[3] = self.calculate_points_for_color(&Color::Green);
            },
        }

        self.turn = turn;

        if self.turn.is_multiple_of(4) {
           self.round += 1;
        }

        const COLOR_ORDER_ONE: [Color; 4] = [Color::Blue, Color::Yellow, Color::Red, Color::Green];
        const COLOR_ORDER_TWO: [Color; 4] = [Color::Yellow, Color::Red, Color::Green, Color::Blue];

        // Current color can be caluclated from turn number
        #[allow(clippy::indexing_slicing)]
        if self.is_starting_team_one {
            self.current_turn_color = COLOR_ORDER_ONE[(self.turn % 4) as usize];
        } else {
           self.current_turn_color = COLOR_ORDER_TWO[(self.turn % 4) as usize];
        }
    }
    
    // Calculates the points for a given color based on the current game state.
    fn calculate_points_for_color(&self, color: &Color) -> u8 {
        let mut points = self.board.get_colored_tiles(color);

        // Extra points for no pieces left
        if self.get_color_pieces(color).is_empty() {
            points += 10;
        
            // Extra points for last piece being mono
            let last_move = self.get_last_move(color);
            #[allow(clippy::unwrap_used)]
            if last_move.is_some() && last_move.unwrap().piece == PieceType::Mono {
                points += 5;
            }
        }

        points
    }

    /// Returns the total score for an entire team.
    pub fn get_points_for_team(&self, team: &crate::game::team::Team) -> u8 {
        let team_colors = team.get_team_colors();
        let mut total_points = 0;

        for color in team_colors.iter() {
            total_points += self.get_points_for_color(color);
        }

        total_points
    }

    /// Returns the accumulated score for a single color.
    pub fn get_points_for_color(&self, color: &Color) -> u8 {
        match color {
            Color::Blue => self.points[0],
            Color::Yellow => self.points[1],
            Color::Red => self.points[2],
            Color::Green => self.points[3],
        }
    }

    /// Returns the last move recorded for the given color, if any.
    pub fn get_last_move(&self, color: &Color) -> &Option<Move> {
        match color {
            Color::Blue => &self.last_move[0],
            Color::Yellow => &self.last_move[1],
            Color::Red => &self.last_move[2],
            Color::Green => &self.last_move[3],
        }
    }

    /// Returns the color whose turn it is.
    pub fn get_current_turn_color(&self) -> &Color {
        &self.current_turn_color
    }
    
    /// Sets the active player color.
    pub fn set_current_turn_color(&mut self, color: Color) {
        self.current_turn_color = color;
    }

    /// Returns the current move number.
    pub fn get_turn(&self) -> &u8 {
        &self.turn
    }

    /// Sets the current move number.
    pub fn set_turn(&mut self, turn: u8) {
        self.turn = turn;
    }

    /// Returns the current round number.
    pub fn get_round(&self) -> &u8 {
        &self.round
    }

    /// Sets the current round number.
    pub fn set_round(&mut self, round: u8) {
        self.round = round;
    }

    /// Returns the starting piece type for the match.
    pub fn get_starting_piece(&self) -> &PieceType {
        &self.starting_piece
    }

    /// Sets the starting piece type.
    pub fn set_starting_piece(&mut self, piece: PieceType) {
        self.starting_piece = piece;
    }

    /// Returns whether team one starts the match.
    pub fn is_starting_team_one(&self) -> &bool {
        &self.is_starting_team_one
    }

    /// Sets whether team one starts the match.
    pub fn set_is_starting_team_one(&mut self, is_starting_team_one: bool) {
        self.is_starting_team_one = is_starting_team_one;
    }

    /// Returns a reference to the current board.
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Replaces the entire board state.
    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }

    /// Returns the remaining piece types for the given color.
    pub fn get_color_pieces(&self, color: &Color) -> &[PieceType] {
        match color {
            Color::Blue => &self.pieces[0],
            Color::Yellow => &self.pieces[1],
            Color::Red => &self.pieces[2],
            Color::Green => &self.pieces[3],
        }
    }

    /// Replaces the remaining pieces of the given color.
    pub fn set_color_pieces(&mut self, color: &Color, pieces: Vec<PieceType>) {
        match color {
            Color::Blue => self.pieces[0] = pieces,
            Color::Yellow => self.pieces[1] = pieces,
            Color::Red => self.pieces[2] = pieces,
            Color::Green => self.pieces[3] = pieces,
        }
    }
}