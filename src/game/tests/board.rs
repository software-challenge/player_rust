use super::Board;
use crate::game::{color::Color, constants::BOARD_SIZE, piece::{Piece, PieceType}, rotation::Rotation};

#[test]
fn test_board_initialization() {
    let board = Board::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            assert_eq!(board.get_cell(col, row), None);
        }
    }
}

#[test]
fn test_set_and_get_cell() {
    let mut board = Board::new();
    assert!(board.set_cell(3, 4, Color::Red));
    assert_eq!(board.get_cell(3, 4), Some(Color::Red));
    assert_eq!(board.set_cell(BOARD_SIZE, BOARD_SIZE, Color::Blue), false); // Out of bounds, should return false
    assert_eq!(board.get_cell(BOARD_SIZE, BOARD_SIZE), None); // Out of bounds, should return None
}

#[test]
fn test_get_colored_tiles() {
    let mut board = Board::new();
    assert_eq!(board.get_colored_tiles(&Color::Red), 0);
    board.set_cell(0, 0, Color::Red);
    board.set_cell(1, 1, Color::Red);
    board.set_cell(2, 2, Color::Blue);
    assert_eq!(board.get_colored_tiles(&Color::Red), 2);
    assert_eq!(board.get_colored_tiles(&Color::Blue), 1);
}

#[test]
fn test_place_piece_unchecked_no_rotation_no_flipping() {
    let mut board = Board::new();
    
    let piece = Piece::new(PieceType::TetroI, Rotation::None, false);
    board.place_piece_unchecked(0, 0, Color::Green, piece);
    assert_eq!(board.get_cell(0, 0), Some(Color::Green));
    assert_eq!(board.get_cell(0, 1), Some(Color::Green));
    assert_eq!(board.get_cell(0, 2), Some(Color::Green));
    assert_eq!(board.get_cell(0, 3), Some(Color::Green));
}

#[test]
fn test_place_piece_unchecked_rotation_no_flipping() {
    let mut board = Board::new();
    
    let piece = Piece::new(PieceType::TetroI, Rotation::Right, false);
    board.place_piece_unchecked(0, 0, Color::Green, piece);
    assert_eq!(board.get_cell(0, 0), Some(Color::Green));
    assert_eq!(board.get_cell(1, 0), Some(Color::Green));
    assert_eq!(board.get_cell(2, 0), Some(Color::Green));
    assert_eq!(board.get_cell(3, 0), Some(Color::Green));
}

#[test]
fn test_place_piece_unchecked_no_rotation_flipping() {
    let mut board = Board::new();
    
    let piece = Piece::new(PieceType::TetroL, Rotation::None, true);
    board.place_piece_unchecked(0, 0, Color::Green, piece);
    assert_eq!(board.get_cell(1, 0), Some(Color::Green));
    assert_eq!(board.get_cell(1, 1), Some(Color::Green));
    assert_eq!(board.get_cell(1, 2), Some(Color::Green));
    assert_eq!(board.get_cell(0, 2), Some(Color::Green));
}

#[test]
fn test_place_piece_unchecked_rotation_flipping() {
    let mut board = Board::new();
    
    let piece = Piece::new(PieceType::TetroL, Rotation::Left, true);
    board.place_piece_unchecked(0, 0, Color::Green, piece);
    assert_eq!(board.get_cell(0, 0), Some(Color::Green));
    assert_eq!(board.get_cell(0, 1), Some(Color::Green));
    assert_eq!(board.get_cell(1, 1), Some(Color::Green));
    assert_eq!(board.get_cell(2, 1), Some(Color::Green));
}

#[test]
fn test_place_piece_success() {
    let mut board = Board::new();
    
    let piece = Piece::new(PieceType::TetroI, Rotation::None, false);
    assert!(board.place_piece(0, 0, Color::Green, piece));
    assert_eq!(board.get_cell(0, 0), Some(Color::Green));
    assert_eq!(board.get_cell(0, 1), Some(Color::Green));
    assert_eq!(board.get_cell(0, 2), Some(Color::Green));
    assert_eq!(board.get_cell(0, 3), Some(Color::Green));
}

#[test]
fn test_place_piece_failure_out_of_bounds() {
    let mut board = Board::new();
    
    let piece = Piece::new(PieceType::TetroI, Rotation::None, false);
    assert!(!board.place_piece(BOARD_SIZE - 1, BOARD_SIZE - 1, Color::Green, piece)); // This should fail as it goes out of bounds
}