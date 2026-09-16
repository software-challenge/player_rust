use crate::game::{
    board::Board,
    color::Color,
    coordinate::Coordinate,
    gamestate::GameState,
    piece::PieceType,
    r#move::Move,
    rotation::Rotation,
};

use super::{
    get_colored_fields, get_possible_moves, get_possible_moves_for_piece, get_valid_fields,
    is_valid_move,
};

fn blue_turn_state_with_board(board: Board, pieces: Vec<PieceType>) -> GameState {
    GameState::new(
        PieceType::Mono,
        true,
        board,
        5,
        2,
        Color::Blue,
        pieces,
        vec![],
        vec![],
        vec![],
    )
}

#[test]
fn invalid_when_directly_adjacent_to_own_piece() {
    let mut board = Board::new();
    board.set_cell(5, 5, Color::Blue);

    let state = blue_turn_state_with_board(board, vec![PieceType::Mono]);
    let m = Move {
        color: Color::Blue,
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
    board.set_cell(5, 5, Color::Blue);

    let state = blue_turn_state_with_board(board, vec![PieceType::Mono]);
    let m = Move {
        color: Color::Blue,
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
    board.set_cell(5, 5, Color::Blue);

    let state = blue_turn_state_with_board(board, vec![PieceType::Mono]);
    let m = Move {
        color: Color::Blue,
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
    board.set_cell(5, 5, Color::Blue);

    let state = GameState::new(
        PieceType::Mono,
        true,
        board,
        5,
        2,
        Color::Blue,
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

fn mono_move(color: Color, x: usize, y: usize) -> Move {
    Move {
        color,
        piece: PieceType::Mono,
        x,
        y,
        is_flipped: false,
        rotation: Rotation::None,
        skip: false,
    }
}

#[test]
fn invalid_when_piece_is_not_available() {
    let state = blue_turn_state_with_board(Board::new(), vec![]);

    assert!(!is_valid_move(&state, &mono_move(Color::Blue, 0, 0)));
}

#[test]
fn invalid_when_placement_is_out_of_bounds() {
    let state = blue_turn_state_with_board(Board::new(), vec![PieceType::Mono]);

    assert!(!is_valid_move(&state, &mono_move(Color::Blue, 20, 0)));
}

#[test]
fn invalid_when_placement_overlaps_an_occupied_cell() {
    let mut board = Board::new();
    board.set_cell(0, 0, Color::Yellow);
    let state = blue_turn_state_with_board(board, vec![PieceType::Mono]);

    assert!(!is_valid_move(&state, &mono_move(Color::Blue, 0, 0)));
}

#[test]
fn skip_move_is_always_valid() {
    let mut move_to_skip = mono_move(Color::Blue, 20, 20);
    move_to_skip.skip = true;
    let state = blue_turn_state_with_board(Board::new(), vec![]);

    assert!(is_valid_move(&state, &move_to_skip));
}

#[test]
fn valid_fields_contain_only_unoccupied_diagonal_corners() {
    let mut board = Board::new();
    board.set_cell(5, 5, Color::Blue);
    board.set_cell(7, 7, Color::Blue);
    board.set_cell(4, 6, Color::Blue);

    let fields = get_valid_fields(&board, &Color::Blue);

    assert!(fields.contains(&Coordinate { x: 6, y: 6 }));
    assert!(!fields.contains(&Coordinate { x: 6, y: 5 }));
    assert!(!fields.contains(&Coordinate { x: 7, y: 7 }));
    assert!(!fields.contains(&Coordinate { x: 5, y: 6 }));
}

#[test]
fn colored_fields_returns_only_matching_cells() {
    let mut board = Board::new();
    board.set_cell(1, 2, Color::Blue);
    board.set_cell(3, 4, Color::Yellow);

    assert_eq!(
        get_colored_fields(&board, &Color::Blue),
        vec![Coordinate { x: 1, y: 2 }]
    );
}

#[test]
fn first_round_generates_starting_piece_moves() {
    let state = GameState::new(
        PieceType::Mono,
        true,
        Board::new(),
        0,
        1,
        Color::Blue,
        vec![PieceType::Mono],
        vec![],
        vec![],
        vec![],
    );

    let moves = get_possible_moves(&state);

    assert!(!moves.is_empty());
    assert!(moves.iter().all(|m| !m.skip));
}