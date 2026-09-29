use crate::game::{
    board::Board,
    color::Color,
    piece::PieceType,
    rotation::Rotation,
    r#move::Move,
    team::Team,
};

use super::GameState;

#[test]
fn test_initialize_game_state() {
    let game_state = GameState::new(
        PieceType::PentoR,
        Default::default(),
        Board::new(),
        0,
        0,
        Color::Blue,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );

    // Check that the initial board is empty
    for row in 0..20 {
        for col in 0..20 {
            assert_eq!(game_state.get_board().get_cell(row, col), None);
        }
    }

    // Check that the current team is Team One
    assert_eq!(*game_state.get_current_turn_color(), Color::Blue);

    // Check that the initial scores are zero
    assert_eq!(game_state.get_points_for_team(&Team::One), 0);
    assert_eq!(game_state.get_points_for_team(&Team::Two), 0);
}

#[test]
fn test_apply_move_unchecked() {
    let mut game_state = GameState::new(
        PieceType::Mono,
        true,
        Board::new(),
        0,
        0,
        Color::Blue,
        vec![PieceType::Mono],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let m = Move::new(Color::Blue, PieceType::Mono, 0, 0, false, Rotation::None, false);

    game_state.apply_move_unchecked(&m, 4);

    assert_eq!(game_state.get_board().get_cell(0, 0), Some(Color::Blue));
    assert!(game_state.get_color_pieces(&Color::Blue).is_empty());
    assert_eq!(game_state.get_last_move(&Color::Blue), &Some(m));
    assert_eq!(game_state.get_points_for_color(&Color::Blue), 16);
    assert_eq!(game_state.get_points_for_team(&Team::One), 16);
    assert_eq!(*game_state.get_turn(), 4);
    assert_eq!(*game_state.get_round(), 1);
    assert_eq!(*game_state.get_current_turn_color(), Color::Blue);
}

#[test]
fn test_apply_move() {
    let mut game_state = GameState::new(
        PieceType::Mono,
        true,
        Board::new(),
        0,
        0,
        Color::Blue,
        vec![PieceType::Mono],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let m = Move::new(Color::Blue, PieceType::Mono, 0, 0, false, Rotation::None, false);

    assert!(game_state.apply_move(&m, 1));
    assert_eq!(game_state.get_board().get_cell(0, 0), Some(Color::Blue));
    assert_eq!(game_state.get_points_for_color(&Color::Blue), 16);
    assert_eq!(*game_state.get_turn(), 1);
    assert_eq!(*game_state.get_round(), 0);
    assert_eq!(*game_state.get_current_turn_color(), Color::Yellow);

    let state_after_success = game_state.clone();
    let invalid_move = Move::new(Color::Blue, PieceType::Mono, 0, 0, false, Rotation::None, false);

    assert!(!game_state.apply_move(&invalid_move, 2));
    assert_eq!(game_state, state_after_success);

}

