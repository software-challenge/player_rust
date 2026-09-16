use super::{Piece, PieceType};
use crate::{game::rotation::Rotation, game::coordinate::Coordinate};

fn sorted_xy(coordinates: Vec<Coordinate>) -> Vec<(isize, isize)> {
    let mut xy: Vec<(isize, isize)> = coordinates
        .into_iter()
        .map(|coord| (coord.x, coord.y))
        .collect();
    xy.sort_unstable();
    xy
}

#[test]
fn pento_r_get_coordinates_all_8_variants() {
    let cases: Vec<(Rotation, bool, Vec<(isize, isize)>)> = vec![
        (
            Rotation::None,
            false,
            vec![(2, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        ),
        (
            Rotation::Right,
            false,
            vec![(2, 2), (1, 0), (1, 1), (1, 2), (0, 1)],
        ),
        (
            Rotation::Mirror,
            false,
            vec![(0, 2), (2, 1), (1, 1), (0, 1), (1, 0)],
        ),
        (
            Rotation::Left,
            false,
            vec![(0, 0), (1, 2), (1, 1), (1, 0), (2, 1)],
        ),
        (
            Rotation::None,
            true,
            vec![(0, 0), (2, 1), (1, 1), (0, 1), (1, 2)],
        ),
        (
            Rotation::Right,
            true,
            vec![(0, 2), (1, 0), (1, 1), (1, 2), (2, 1)],
        ),
        (
            Rotation::Mirror,
            true,
            vec![(2, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
        ),
        (
            Rotation::Left,
            true,
            vec![(2, 0), (1, 2), (1, 1), (1, 0), (0, 1)],
        ),
    ];

    for (rotation, is_flipped, expected) in cases {
        let piece = Piece::new(PieceType::PentoR, rotation, is_flipped);
        let actual = sorted_xy(piece.get_coordinates());

        assert_eq!(
            actual,
            sorted_xy(
                expected
                    .into_iter()
                    .map(|(x, y)| Coordinate::new(x, y))
                    .collect()
            ),
            "unexpected coordinates for rotation={:?}, is_flipped={}",
            rotation,
            is_flipped
        );
    }
}

#[test]
fn pento_r_all_variants_generates_all_unique_orientations() {
    let variants = PieceType::PentoR.all_variants(true);
    let unfiltered = PieceType::PentoR.all_variants(false);

    assert_eq!(variants.len(), 8, "filtered variants should still include all unique PentoR states");
    assert_eq!(unfiltered.len(), 8, "every rotation/flip combination should be represented");

    let expected_rotations = vec![
        Rotation::None,
        Rotation::Right,
        Rotation::Mirror,
        Rotation::Left,
    ];
    let mut seen = Vec::new();
    for (_, (rotation, is_flipped)) in variants {
        seen.push((rotation, is_flipped));
    }
    for rotation in expected_rotations {
        for is_flipped in [false, true] {
            assert!(
                seen.contains(&(rotation, is_flipped)),
                "missing rotation={:?}, is_flipped={}",
                rotation,
                is_flipped
            );
        }
    }
}

#[test]
fn piece_mutators_keep_state_in_sync() {
    let mut piece = Piece::new(PieceType::Mono, Rotation::None, false);

    assert_eq!(piece.get_piece_type(), &PieceType::Mono);
    assert_eq!(piece.get_rotation(), &Rotation::None);
    assert!(piece.is_flipped() == &false);

    piece.set_piece_type(PieceType::PentoR);
    piece.set_rotation(Rotation::Right);
    piece.set_flipped(true);

    assert_eq!(piece.get_piece_type(), &PieceType::PentoR);
    assert_eq!(piece.get_rotation(), &Rotation::Right);
    assert!(piece.is_flipped() == &true);
    assert_eq!(piece.get_coordinates().len(), 5);
}

#[test]
fn piece_type_parse_round_trip_for_all_variants() {
    let values = [
        PieceType::Mono,
        PieceType::Domino,
        PieceType::TrioL,
        PieceType::TrioI,
        PieceType::TetroO,
        PieceType::TetroT,
        PieceType::TetroI,
        PieceType::TetroL,
        PieceType::TetroZ,
        PieceType::PentoL,
        PieceType::PentoT,
        PieceType::PentoV,
        PieceType::PentoS,
        PieceType::PentoZ,
        PieceType::PentoI,
        PieceType::PentoP,
        PieceType::PentoW,
        PieceType::PentoU,
        PieceType::PentoR,
        PieceType::PentoX,
        PieceType::PentoY,
    ];

    for piece_type in values {
        let parsed = piece_type.to_string().parse::<PieceType>();
        assert_eq!(parsed, Ok(piece_type), "round-trip parse failed for {}", piece_type);
    }
}