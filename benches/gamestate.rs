use socha:: games::active::{board::Board, color::Color, gamestate::GameState, r#move::{Move, SetMove}, piece::{self, PieceType}, rotation::Rotation};

use criterion::{
  criterion_group, 
  criterion_main, 
  Criterion
};

fn gamestate_apply_move_unchecked_fn(mut gamestate: GameState, m: Move) {
    gamestate.apply_move_unchecked(&m, 1);
}

fn gamestate_get_color_pieces(gamestate: GameState, color: Color) {
    let _ = gamestate.get_color_pieces(color);
}

fn gamestate_result_benchmark(c: &mut Criterion) {

    let gamestate = GameState::new(PieceType::PentoR, true, Board::new(), 0, 0, Color::Blue, piece::ALL_PIECE_TYPES.to_vec(), piece::ALL_PIECE_TYPES.to_vec(), piece::ALL_PIECE_TYPES.to_vec(), piece::ALL_PIECE_TYPES.to_vec()); 

    c.bench_function("gamestate_place_apply_move_pento_r_no_rotation_no_flipping_0_0", |b| b.iter(|| gamestate_apply_move_unchecked_fn(gamestate.clone(), Move::Set(SetMove::new(Color::Blue, PieceType::PentoR, 0, 0, false, Rotation::None)))));


    c.bench_function("gamestate_get_color_pieces_blue", |b| {
        b.iter(|| gamestate_get_color_pieces(gamestate.clone(), Color::Blue));
    });
    c.bench_function("gamestate_get_color_pieces_yellow", |b| {
        b.iter(|| gamestate_get_color_pieces(gamestate.clone(), Color::Yellow));
    });
    c.bench_function("gamestate_get_color_pieces_red", |b| {
        b.iter(|| gamestate_get_color_pieces(gamestate.clone(), Color::Red));
    });
    c.bench_function("gamestate_get_color_pieces_green", |b| {
        b.iter(|| gamestate_get_color_pieces(gamestate.clone(), Color::Green));
    });

}

criterion_group!(benches, gamestate_result_benchmark);
criterion_main!(benches);