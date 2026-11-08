use std::hint::black_box;

use socha::games::active::{
    board::Board,
    color::Color,
    gamestate::GameState,
    piece::{self, PieceType},
    r#move::{Move, SetMove},
    rotation::Rotation,
};

use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn gamestate_apply_move_unchecked_fn(mut gamestate: GameState, m: Move) {
    gamestate.apply_move_unchecked(&m, 1);
}

fn gamestate_update_gamestate_fn(mut gamestate: GameState, m: Move) {
    gamestate.apply_move_unchecked(&m, 1);
}

fn gamestate_get_color_pieces(gamestate: GameState, color: Color) {
    let _ = gamestate.get_color_pieces(color);
}

fn gamestate_with_remaining_piece_count(remaining_piece_count: usize) -> GameState {
    let blue_pieces = piece::ALL_PIECE_TYPES
        .iter()
        .take(remaining_piece_count)
        .copied()
        .collect();

    GameState::new(
        PieceType::Mono,
        true,
        Board::new(),
        0,
        0,
        Color::Blue,
        blue_pieces,
        piece::ALL_PIECE_TYPES.to_vec(),
        piece::ALL_PIECE_TYPES.to_vec(),
        piece::ALL_PIECE_TYPES.to_vec(),
    )
}

fn gamestate_result_benchmark(c: &mut Criterion) {
    let gamestate = GameState::new(
        PieceType::PentoR,
        true,
        Board::new(),
        0,
        0,
        Color::Blue,
        piece::ALL_PIECE_TYPES.to_vec(),
        piece::ALL_PIECE_TYPES.to_vec(),
        piece::ALL_PIECE_TYPES.to_vec(),
        piece::ALL_PIECE_TYPES.to_vec(),
    );

    c.bench_function(
        "gamestate_place_apply_move_pento_r_no_rotation_no_flipping_0_0",
        |b| {
            b.iter(|| {
                gamestate_apply_move_unchecked_fn(
                    gamestate.clone(),
                    Move::Set(SetMove::new(
                        Color::Blue,
                        PieceType::PentoR,
                        0,
                        0,
                        false,
                        Rotation::None,
                    )),
                )
            })
        },
    );

    c.bench_function(
        "gamestate_update_gamestate_pento_r_no_rotation_no_flipping_0_0",
        |b| {
            b.iter(|| {
                gamestate_update_gamestate_fn(
                    gamestate.clone(),
                    Move::Set(SetMove::new(
                        Color::Blue,
                        PieceType::PentoR,
                        0,
                        0,
                        false,
                        Rotation::None,
                    )),
                )
            });
        },
    );

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

    let mut removal_group = c.benchmark_group("gamestate_remove_piece_by_remaining_count");
    for remaining_piece_count in [21, 11, 1] {
        let state = gamestate_with_remaining_piece_count(remaining_piece_count);
        let move_to_apply = Move::Set(SetMove::new(
            Color::Blue,
            PieceType::Mono,
            0,
            0,
            false,
            Rotation::None,
        ));

        removal_group.bench_with_input(
            BenchmarkId::from_parameter(remaining_piece_count),
            &state,
            |b, state| {
                b.iter_batched_ref(
                    || state.clone(),
                    |state| {
                        state.apply_move_unchecked(black_box(&move_to_apply), 1);
                        black_box(state);
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
    removal_group.finish();
}

criterion_group!(benches, gamestate_result_benchmark);
criterion_main!(benches);
