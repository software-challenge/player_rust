use crate::{game::{self, board::{self, Board, Team}, constants, gamestate::GameState, r#move::{Move, Rotation}, piece::PieceType}, util::coordinate::Coordinate};

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
        let piece_moves = get_possible_moves_for_piece(gamestate, &piece, valid_fields.clone());
        moves.extend(piece_moves);
    }

    moves
}

/// Returns a vector of all possible moves for the given piece in the given game state
/// Only returns valid moves after round 1!
pub fn get_possible_moves_for_piece(gamestate: &GameState, piece: &PieceType, valid_fields: Vec<Coordinate>) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    for field in valid_fields {
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

            let m = Move {
                team: gamestate.current_turn_team,
                piece: *piece,
                x: field.x as usize,
                y: field.y as usize,
                is_flipped,
                rotation,
                skip: false,
            };

            if is_valid_move(gamestate, &m) {moves.push(m)}
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

    // Get the base coordinates of the piece
    let base_coordinates = m.piece.base_coordinates().to_vec();

    // Transform the coordinates based on rotation and flip
    let mut transformed_coordinates: Vec<Coordinate> = base_coordinates.clone();
    for coord in &mut transformed_coordinates {
        if m.is_flipped {
            coord.flip_on_vertical();
        }
        coord.rotate(&m.rotation);
    }

    // Normalize the coordinates to start from (0, 0)
    let min_x = transformed_coordinates.iter().map(|c| c.x).min().unwrap();
    let min_y = transformed_coordinates.iter().map(|c| c.y).min().unwrap();
    for coord in &mut transformed_coordinates {
        coord.subtract(&Coordinate { x: min_x, y: min_y });
    }

    // Check if all coordinates are within bounds and not occupied
    for coord in &transformed_coordinates {
        let board_x = (m.x as isize + coord.x) as usize;
        let board_y = (m.y as isize + coord.y) as usize;

        if board_x >= constants::BOARD_WIDTH as usize || board_y >= constants::BOARD_HEIGHT as usize {
            return false; // Out of bounds
        }

        if gamestate.board.get_cell(board_x, board_y).is_some() {
            return false; // Cell is already occupied
        }
    }

    true
}
