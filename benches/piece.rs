use criterion::{
  criterion_group, 
  criterion_main, 
  Criterion
};

use socha::games::{active::{piece::{self, Piece}, rotation::Rotation}};

fn piece_to_coordinates(piece: Piece) {
    let _ = piece.get_coordinates();
}

fn piece_result_benchmark(c: &mut Criterion) {
    c.bench_function("piece_to_coordinate_benchmark_pento_r_rotated_flipped", |b| b.iter(|| piece_to_coordinates(Piece::new(piece::PieceType::PentoR, Rotation::Right, true))));
    c.bench_function("piece_to_coordinate_benchmark_pento_r_flipped", |b| b.iter(|| piece_to_coordinates(Piece::new(piece::PieceType::PentoR, Rotation::None, true))));
    c.bench_function("piece_to_coordinate_benchmark_pento_r_rotated", |b| b.iter(|| piece_to_coordinates(Piece::new(piece::PieceType::PentoR, Rotation::Right, false))));
    c.bench_function("piece_to_coordinate_benchmark_pento_r", |b| b.iter(|| piece_to_coordinates(Piece::new(piece::PieceType::PentoR, Rotation::None, false))));
}

criterion_group!(benches, piece_result_benchmark);
criterion_main!(benches);