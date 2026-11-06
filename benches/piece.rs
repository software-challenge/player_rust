use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use socha::games::active::{
    piece::{ALL_PIECE_TYPES, Piece, PieceType},
    rotation::Rotation,
};

fn piece_to_coordinates(piece: Piece) {
    let _ = piece.get_coordinates();
}

fn piece_result_benchmark(c: &mut Criterion) {
    c.bench_function(
        "piece_to_coordinate_benchmark_pento_r_rotated_flipped",
        |b| b.iter(|| piece_to_coordinates(Piece::new(PieceType::PentoR, Rotation::Right, true))),
    );
    c.bench_function("piece_to_coordinate_benchmark_pento_r_flipped", |b| {
        b.iter(|| piece_to_coordinates(Piece::new(PieceType::PentoR, Rotation::None, true)))
    });
    c.bench_function("piece_to_coordinate_benchmark_pento_r_rotated", |b| {
        b.iter(|| piece_to_coordinates(Piece::new(PieceType::PentoR, Rotation::Right, false)))
    });
    c.bench_function("piece_to_coordinate_benchmark_pento_r", |b| {
        b.iter(|| piece_to_coordinates(Piece::new(PieceType::PentoR, Rotation::None, false)))
    });

    for filter in [false, true] {
        let filter_name = if filter { "filtered" } else { "unfiltered" };
        c.bench_function(
            &format!("piece_type_all_variants/{filter_name}_all_types"),
            |b| {
                b.iter(|| {
                    for piece_type in ALL_PIECE_TYPES {
                        black_box(black_box(piece_type).all_variants(filter));
                    }
                })
            },
        );
    }
}

criterion_group!(benches, piece_result_benchmark);
criterion_main!(benches);
