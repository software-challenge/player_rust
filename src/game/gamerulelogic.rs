use std::collections::HashSet;

use crate::{game::{board::{Board, Team}, constants, gamestate::GameState, r#move::Move, piece::{Piece, PieceType}}, util::coordinate::Coordinate};

/// Returns a vector of all possible moves for the current team in the given game state
/// Does not include skip moves
pub fn get_possible_moves(gamestate: &GameState) -> Vec<Move> {
    if gamestate.round == 1 {
        return get_possible_start_moves(gamestate);
    }
    return get_possible_set_moves(gamestate);
}

pub fn get_possible_start_moves(gamestate: &GameState) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let piece: &PieceType = &gamestate.starting_piece;

    for variant in piece.all_variants() {
        let (relative_coordinates, (rotation, is_flipped)) = variant;

        // Calculate the bounding box of the piece variant
        let mut min_x = std::isize::MAX;
        let mut min_y = std::isize::MAX;
        let mut max_x = std::isize::MIN;
        let mut max_y = std::isize::MIN;

        for coord in &relative_coordinates {
            if coord.x < min_x {
                min_x = coord.x;
            }
            if coord.y < min_y {
                min_y = coord.y;
            }
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.y > max_y {
                max_y = coord.y;
            }
        }

        println!("Variant: {} {} {} {}", piece.to_string(), is_flipped, rotation.to_string(), relative_coordinates.len());
        println!("Bounding box: min_x: {}, min_y: {}, max_x: {}, max_y: {}", min_x, min_y, max_x, max_y);

        // Add all possible border placements
        for x in 0..(constants::BOARD_WIDTH - max_x) {
            let mut m = Move {
                team: gamestate.current_turn_team,
                piece: *piece,
                x: x as usize,
                y: 0,
                is_flipped,
                rotation,
                skip: false,
            };

            if is_valid_move(gamestate, &m) {moves.push(m)}

            m = Move {
                team: gamestate.current_turn_team,
                piece: *piece,
                x: x as usize,
                y: (constants::BOARD_HEIGHT - max_y) as usize,
                is_flipped,
                rotation,
                skip: false,
            };

            if is_valid_move(gamestate, &m) {moves.push(m)}
        }

        for y in 0..(constants::BOARD_WIDTH - max_y) {
            let mut m = Move {
                team: gamestate.current_turn_team,
                piece: *piece,
                x: 0,
                y: y as usize,
                is_flipped,
                rotation,
                skip: false,
            };

            if is_valid_move(gamestate, &m) {moves.push(m)}

            m = Move {
                team: gamestate.current_turn_team,
                piece: *piece,
                x: (constants::BOARD_WIDTH - max_x) as usize,
                y: y as usize,
                is_flipped,
                rotation,
                skip: false,
            };

            if is_valid_move(gamestate, &m) {moves.push(m)}
        }
    }

    moves
}

pub fn get_possible_set_moves(gamestate: &GameState) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let valid_fields: Vec<Coordinate> = get_valid_fields(&gamestate.board, &gamestate.current_turn_team);

    for piece in gamestate.get_team_pieces(&gamestate.current_turn_team) {
        let piece_moves = get_possible_moves_for_piece(gamestate, piece, &valid_fields);
        moves.extend(piece_moves);
    }

    moves
}

/// Returns a vector of all possible moves for the given piece in the given game state
/// Only returns valid moves after round 1!
pub fn get_possible_moves_for_piece(gamestate: &GameState, piece: &PieceType, valid_fields: &[Coordinate]) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let mut seen: HashSet<(usize, usize, bool, crate::game::r#move::Rotation)> = HashSet::new();

    for field in valid_fields {
        for variant in piece.all_variants() {
            let (relative_coordinates, (rotation, is_flipped)) = variant;

            // Align each block of the variant to this candidate corner field.
            for anchor in &relative_coordinates {
                let origin_x = field.x - anchor.x;
                let origin_y = field.y - anchor.y;

                if origin_x < 0 || origin_y < 0 {
                    continue;
                }

                let m = Move {
                    team: gamestate.current_turn_team,
                    piece: *piece,
                    x: origin_x as usize,
                    y: origin_y as usize,
                    is_flipped,
                    rotation,
                    skip: false,
                };

                if !seen.insert((m.x, m.y, m.is_flipped, m.rotation)) {
                    continue;
                }

                if is_valid_move(gamestate, &m) {
                    moves.push(m)
                }
            }
        }
    }

    moves
}

/// Return a vector of all coordinates on the board that are valid for the given team to place a piece on
pub fn get_valid_fields(board: &Board, team: &Team) -> Vec<Coordinate> {

    let mut valid_fields: Vec<Coordinate> = Vec::new();

    for colored_field in get_colored_fiels(board, team) {
        for dx in [-1, 1] {
            for dy in [-1, 1] {
                let corner = Coordinate {
                    x: colored_field.x + dx,
                    y: colored_field.y + dy,
                };

                // Check if the corner is already in the valid_fields vector
                if valid_fields.iter().any(|&c| c.x == corner.x && c.y == corner.y) {
                    continue;
                }

                // Check if the corner is within boounds
                if corner.x < 0
                    || corner.x >= constants::BOARD_WIDTH
                    || corner.y < 0
                    || corner.y >= constants::BOARD_HEIGHT
                {
                    continue;
                }

                // Check if the corner is already occupied
                if board.get_cell(corner.x as usize, corner.y as usize).is_some() {
                    continue;
                }

                //Check if the corner is adjacent to any piece of the same team
                if [
                    Coordinate { x: corner.x - 1, y: corner.y },
                    Coordinate { x: corner.x + 1, y: corner.y },
                    Coordinate { x: corner.x, y: corner.y - 1 },
                    Coordinate { x: corner.x, y: corner.y + 1 },
                ]
                .iter()
                .any(|neighbor| {
                    neighbor.x >= 0
                        && neighbor.x < constants::BOARD_WIDTH
                        && neighbor.y >= 0
                        && neighbor.y < constants::BOARD_HEIGHT
                        && board.get_cell(neighbor.x as usize, neighbor.y as usize) == Some(*team)
                })
                {
                    continue;
                }

                valid_fields.push(corner);
            }
        }
    }

    valid_fields
}

/// Returns a vector of all coordinates on the board that are occupied by any piece of the given team
pub fn get_colored_fiels(board: &Board, team: &Team) -> Vec<Coordinate> {
    let mut colored_fields: Vec<Coordinate> = vec![];

    for y in 0..constants::BOARD_HEIGHT {
        for x in 0..constants::BOARD_WIDTH {
            if board.get_cell(x as usize, y as usize) == Some(*team) {
                colored_fields.push(Coordinate { x: x as isize, y: y as isize });
            }
        }
    }

    colored_fields
}

/// Returns true if the given move is valid in the given game state
/// Does not check for round 1 validity! Only returns true results after round 1! For round 1 use get_possible_start_moves() to get all valid moves
pub fn is_valid_move(gamestate: &GameState, m: &Move) -> bool {
    // Check if the move is a skip move
    if m.skip {
        return true;
    }

    // Check if team has the piece available
    let team_pieces = gamestate.get_team_pieces(&m.team);
    if !team_pieces.contains(&m.piece) {
        return false;
    }

    // Check if all coordinates are within bounds and not occupied.
    let transformed_coordinates: Vec<Coordinate> = (Piece::new(m.piece, m.rotation, m.is_flipped)).get_coordinates();
    let mut placed_cells: Vec<Coordinate> = Vec::with_capacity(transformed_coordinates.len());

    for coord in &transformed_coordinates {
        let board_x = m.x as isize + coord.x;
        let board_y = m.y as isize + coord.y;

        if board_x < 0
            || board_x >= constants::BOARD_WIDTH
            || board_y < 0
            || board_y >= constants::BOARD_HEIGHT
        {
            return false; // Out of bounds
        }

        if gamestate.board.get_cell(board_x as usize, board_y as usize).is_some() {
            return false; // Cell is already occupied
        }

        placed_cells.push(Coordinate { x: board_x, y: board_y });
    }

    let mut has_corner_contact = false;

    for cell in &placed_cells {
        // Direct edge-contact with own pieces is not allowed.
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = cell.x + dx;
            let ny = cell.y + dy;

            if nx < 0 || nx >= constants::BOARD_WIDTH || ny < 0 || ny >= constants::BOARD_HEIGHT {
                continue;
            }

            if gamestate.board.get_cell(nx as usize, ny as usize) == Some(m.team) {
                return false;
            }
        }

        // A legal non-initial move must touch an own piece at a corner.
        for (dx, dy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
            let nx = cell.x + dx;
            let ny = cell.y + dy;

            if nx < 0 || nx >= constants::BOARD_WIDTH || ny < 0 || ny >= constants::BOARD_HEIGHT {
                continue;
            }

            if gamestate.board.get_cell(nx as usize, ny as usize) == Some(m.team) {
                has_corner_contact = true;
            }
        }
    }

    // Only enforce corner contact once the team has at least one tile on the board.
    if !get_colored_fiels(&gamestate.board, &m.team).is_empty() && !has_corner_contact {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::game::{
        board::{Board, Team},
        gamestate::GameState,
        piece::PieceType,
        r#move::{Move, Rotation},
    };
    use crate::util::coordinate::Coordinate;

    use super::{get_possible_moves_for_piece, is_valid_move};

    fn blue_turn_state_with_board(board: Board) -> GameState {
        GameState::new(
            PieceType::Mono,
            board,
            5,
            2,
            Team::Blue,
            vec![PieceType::Mono],
            vec![],
            vec![],
            vec![],
        )
    }

    #[test]
    fn invalid_when_directly_adjacent_to_own_piece() {
        let mut board = Board::new();
        board.board[5][5] = Some(Team::Blue);

        let state = blue_turn_state_with_board(board);
        let m = Move {
            team: Team::Blue,
            piece: PieceType::Mono,
            x: 6,
            y: 5,
            is_flipped: false,
            rotation: Rotation::None,
            skip: false,
        };

        assert!(!is_valid_move(&state, &m));
    }

    #[test]
    fn valid_when_only_corner_contact_exists() {
        let mut board = Board::new();
        board.board[5][5] = Some(Team::Blue);

        let state = blue_turn_state_with_board(board);
        let m = Move {
            team: Team::Blue,
            piece: PieceType::Mono,
            x: 6,
            y: 6,
            is_flipped: false,
            rotation: Rotation::None,
            skip: false,
        };

        assert!(is_valid_move(&state, &m));
    }

    #[test]
    fn invalid_when_no_corner_contact_after_first_move() {
        let mut board = Board::new();
        board.board[5][5] = Some(Team::Blue);

        let state = blue_turn_state_with_board(board);
        let m = Move {
            team: Team::Blue,
            piece: PieceType::Mono,
            x: 10,
            y: 10,
            is_flipped: false,
            rotation: Rotation::None,
            skip: false,
        };

        assert!(!is_valid_move(&state, &m));
    }

    #[test]
    fn calculates_moves_where_corner_is_not_piece_origin() {
        let mut board = Board::new();
        board.board[5][5] = Some(Team::Blue);

        let state = GameState::new(
            PieceType::Mono,
            board,
            5,
            2,
            Team::Blue,
            vec![PieceType::PentoX],
            vec![],
            vec![],
            vec![],
        );

        let valid_fields = vec![Coordinate { x: 6, y: 6 }];
        let moves = get_possible_moves_for_piece(&state, &PieceType::PentoX, &valid_fields);

        assert!(
            moves.iter().any(|m| m.x == 6 && m.y == 5),
            "expected a placement that aligns a non-origin PENTO_X tile to the corner"
        );
    }
}
