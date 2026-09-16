use super::{Coordinate, normalize_coordinates, rotate_coordinates, flip_coordinates};
use crate::game::rotation::Rotation;

#[test]
fn test_coordinate_equality() {
    let coord1 = Coordinate::new(3, 4);
    let coord2 = Coordinate::new(3, 4);
    let coord3 = Coordinate::new(5, 6);

    assert_eq!(coord1, coord2);
    assert_ne!(coord1, coord3);
}

#[test]
fn test_coordinate_addition() {
    let mut coord1 = Coordinate::new(2, 3);
    let coord2 = Coordinate::new(4, 5);
    coord1.add(&coord2);
    assert_eq!(coord1, Coordinate::new(6, 8));
}

#[test]
fn test_coordinate_subtraction() {
    let mut coord1 = Coordinate::new(5, 7);
    let coord2 = Coordinate::new(2, 3);
    coord1.subtract(&coord2);
    assert_eq!(coord1, Coordinate::new(3, 4));
}

#[test]
fn test_coordinate_multiplication() {
    let mut coord = Coordinate::new(3, 4);
    coord.multiply(2);
    assert_eq!(coord, Coordinate::new(6, 8));
}

#[test]
fn test_coordinate_division() {
    let mut coord = Coordinate::new(8, 12);
    coord.divide(4);
    assert_eq!(coord, Coordinate::new(2, 3));
}

#[test]
fn test_coordinate_rotation() {
    let coord = Coordinate::new(1, 2);
    assert_eq!(coord.rotate(&Rotation::Right), Coordinate::new(-2, 1));
    assert_eq!(coord.rotate(&Rotation::Mirror), Coordinate::new(-1, -2));
    assert_eq!(coord.rotate(&Rotation::Left), Coordinate::new(2, -1));
    assert_eq!(coord.rotate(&Rotation::None), coord);
}

#[test]
fn test_coordinate_flip_on_vertical() {
    let coord = Coordinate::new(1, 2);
    assert_eq!(coord.flip_on_vertical(), Coordinate::new(-1, 2));
}

#[test]
fn test_normalize_coordinates() {
    let coords = vec![
        Coordinate::new(3, 4),
        Coordinate::new(1, 2),
        Coordinate::new(5, 6),
    ];
    let normalized = normalize_coordinates(&coords);
    assert_eq!(normalized, vec![
        Coordinate::new(2, 2),
        Coordinate::new(0, 0),
        Coordinate::new(4, 4),
    ]);
}

#[test]
fn test_rotate_coordinates() {
    let coords = vec![
        Coordinate::new(1, 2),
        Coordinate::new(3, 4),
    ];
    let rotated = rotate_coordinates(coords, &Rotation::Right);
    assert_eq!(rotated, vec![
        Coordinate::new(-2, 1),
        Coordinate::new(-4, 3),
    ]);
}

#[test]
fn test_flip_coordinates() {
    let coords = vec![
        Coordinate::new(1, 2),
        Coordinate::new(3, 4),
    ];
    let flipped = flip_coordinates(coords);
    assert_eq!(flipped, vec![
        Coordinate::new(-1, 2),
        Coordinate::new(-3, 4),
    ]);
}