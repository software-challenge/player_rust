use crate::{game::{board::{self, Board, Team}, gamestate::GameState, r#move::{Move, Rotation}}, util::coordinate};

pub fn get_legal_moves_for_team(gamestate: &GameState, team: &Team) -> Vec<Move> {

    let mut moves: Vec<Move> = vec![];

    // For the first round calculate all moves for the starting piece where the staring piece has a block on the 
    if gamestate.turn < 4 {
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
                        rotation
                    });

                    moves.push(Move {
                        team: *team,
                        piece: gamestate.starting_piece,
                        x,
                        y: board::BOARD_HEIGHT - height as usize,
                        is_flipped: flipped,
                        rotation
                    });
                }

                for y in 0..(board::BOARD_HEIGHT - height as usize) {
                    moves.push(Move {
                        team: *team,
                        piece: gamestate.starting_piece,
                        x: 0,
                        y,
                        is_flipped: flipped,
                        rotation
                    });

                    moves.push(Move {
                        team: *team,
                        piece: gamestate.starting_piece,
                        x: board::BOARD_WIDTH - width as usize,
                        y,
                        is_flipped: flipped,
                        rotation
                    });
                }
            }
        }

        return moves;
    }

    return moves;
}

/*
for flip in [true, false] {
            for rotation in [Rotation::None, Rotation::Right, Rotation::Mirror, Rotation::Left] {
                
                let mut transformed_coordinates = base_coordinates.clone();
                if flip {
                    for coordinate in &mut transformed_coordinates {
                        coordinate.flip_on_vertical();
                        coordinate.rotate(&rotation);
                    }
                }

                

            }
        } */