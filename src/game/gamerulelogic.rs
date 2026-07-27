use crate::{game::{board::{self, Team}, gamestate::GameState, r#move::{Move, Rotation}}, util::coordinate::{Coordinate}};

pub fn get_legal_moves_for_team(gamestate: &GameState, team: &Team) -> Vec<Move> {

    let mut moves: Vec<Move> = vec![];

    // For the first round calculate all moves for the starting piece where the staring piece has a block on the 
    if gamestate.round == 1 && gamestate.turn < 4 {
        let base_coordinates = gamestate.starting_piece.base_coordinates().to_vec();

        let mut width = 0;
        let mut height = 0;

        for coord in base_coordinates {
            if coord.x > width {width = coord.x}
            if coord.y > height {height = coord.y}
        }

        for flipped in [true, false] {
            for rotation in [Rotation::None, Rotation::Right, Rotation::Mirror, Rotation::Left] {
                println!("Move");
                
                for x in 0..(board::BOARD_WIDTH - width as usize) {
                    moves.push(Move {
                        team: *team,
                        piece: gamestate.starting_piece,
                        x,
                        y: 0,
                        is_flipped: flipped,
                        rotation,
                        skip: false
                    });

                    moves.push(Move {
                        team: *team,
                        piece: gamestate.starting_piece,
                        x,
                        y: board::BOARD_HEIGHT - height as usize,
                        is_flipped: flipped,
                        rotation,
                        skip: false,
                    });
                }

                for y in 0..(board::BOARD_HEIGHT - height as usize) {
                    moves.push(Move {
                        team: *team,
                        piece: gamestate.starting_piece,
                        x: 0,
                        y,
                        is_flipped: flipped,
                        rotation,
                        skip: false
                    });

                    moves.push(Move {
                        team: *team,
                        piece: gamestate.starting_piece,
                        x: board::BOARD_WIDTH - width as usize,
                        y,
                        is_flipped: flipped,
                        rotation,
                        skip: false
                    });
                }
            }
        }

        return moves;
    }

    // Calculate legal moves for all  
    // TODO: Not all possible moves are captured and really inefficient   
    let anchor_points = find_anchor_points_for_team(gamestate, team);
    let team_pieces = gamestate.get_team_pieces(team);

    for anchor_point in anchor_points {
        for piece in team_pieces {
            let base_coordinates = piece.base_coordinates().to_vec();

            let mut width = 0;
            let mut height = 0;

            for coord in &base_coordinates {
                if coord.x > width {width = coord.x}
                if coord.y > height {height = coord.y}
            }

            for flipped in [true, false] {
                for rotation in [Rotation::None, Rotation::Right, Rotation::Mirror, Rotation::Left] {
                    for x in 0..(board::BOARD_WIDTH - width as usize) {
                        for y in 0..(board::BOARD_HEIGHT - height as usize) {
                            
                            let mut transformed_coordinates: Vec<Coordinate> = base_coordinates.clone();
                            for coord in &mut transformed_coordinates {
                                if flipped {
                                    coord.flip_on_vertical();
                                }

                                coord.rotate(&rotation);
                            }

                            // Normalize the coordinates to start from (0, 0)
                            let min_x = transformed_coordinates.iter().map(|c| c.x).min().unwrap();
                            let min_y = transformed_coordinates.iter().map(|c| c.y).min().unwrap();
                            for coord in &mut transformed_coordinates {
                                coord.subtract(&Coordinate { x: min_x, y: min_y });
                            }

                            // Check if all coordinates are free and have no adjacent pieces of the same team
                            let mut is_valid_move = true;
                            
                            // Temporary: Check if top left is a corner of the piece
                            if !transformed_coordinates.iter().any(|c| c.x == 0 && c.y == 0) {
                                is_valid_move = false;
                            }

                            for coord in &transformed_coordinates {
                                let board_x = (anchor_point.x + coord.x) as usize;
                                let board_y = (anchor_point.y + coord.y) as usize;

                                if gamestate.board.get_cell(board_x, board_y).is_some() {
                                    is_valid_move = false;
                                    break;
                                }

                                // Check adjacent cells for the same team
                                let adjacent_coords = vec![
                                    Coordinate { x: board_x as isize - 1, y: board_y as isize },
                                    Coordinate { x: board_x as isize + 1, y: board_y as isize },
                                    Coordinate { x: board_x as isize, y: board_y as isize - 1 },
                                    Coordinate { x: board_x as isize, y: board_y as isize + 1 },
                                ];

                                for adj in adjacent_coords {
                                    if adj.x >= 0 && adj.x < board::BOARD_WIDTH as isize && adj.y >= 0 && adj.y < board::BOARD_HEIGHT as isize {
                                        if let Some(adj_team) = gamestate.board.get_cell(adj.x as usize, adj.y as usize) {
                                            if adj_team == *team {
                                                is_valid_move = false;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }

                            if is_valid_move {
                                moves.push(Move {
                                    team: *team,
                                    piece: *piece,
                                    x,
                                    y,
                                    is_flipped: flipped,
                                    rotation,
                                    skip:false,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    return moves;
}

pub fn find_anchor_points_for_team(gamestate: &GameState, team: &Team) -> Vec<Coordinate> {
    let mut anchor_points: Vec<Coordinate> = vec![];

    for y in 0..board::BOARD_HEIGHT {
        for x in 0..board::BOARD_WIDTH {
            if let Some(cell_team) = gamestate.board.get_cell(x, y) {
                if cell_team == *team {
                    
                    for dx in [-1, 1] {
                        for dy in [-1, 1] {
                            if (dx as isize).abs() + (dy as isize).abs() == 1 { // Only consider orthogonal neighbors
                                let neighbor_x = x as isize + dx;
                                let neighbor_y = y as isize + dy;

                                if neighbor_x >= 0 && neighbor_x < board::BOARD_WIDTH as isize && neighbor_y >= 0 && neighbor_y < board::BOARD_HEIGHT as isize {
                                    if gamestate.board.get_cell(neighbor_x as usize, neighbor_y as usize).is_none() {
                                        anchor_points.push(Coordinate { x: neighbor_x, y: neighbor_y });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return anchor_points;
}