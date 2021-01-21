use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gol_core::{Board, ConwayState, ConwayStrategy, GridPoint1D, StandardBoardFactory};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

const SHAPES_1D_SMALL: [usize; 14] = [
    3, 10, 20, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
];
const SHAPES_1D_LARGE: [usize; 11] = [
    1, 10_000, 20_000, 30_000, 40_000, 50_000, 60_000, 70_000, 80_000, 90_000, 100_000,
];

type ConwayBoard<CI> = Arc<Mutex<Box<dyn Board<ConwayState, CI, std::vec::IntoIter<CI>>>>>;

fn gen_board_1d(shape: &usize) -> (ConwayBoard<GridPoint1D<i32>>, ConwayBoard<GridPoint1D<i32>>) {
    let strategy = Box::new(ConwayStrategy::new());
    let board_surround = StandardBoardFactory::new_binary_1d_grid(
        shape.clone(),
        ConwayState::Dead,
        ConwayState::Alive,
        1,
        &HashSet::new(),
        strategy,
        Vec::new(),
        false,
    );

    let strategy = Box::new(ConwayStrategy::new());
    let board_donut = StandardBoardFactory::new_binary_1d_grid(
        shape.clone(),
        ConwayState::Dead,
        ConwayState::Alive,
        1,
        &HashSet::new(),
        strategy,
        Vec::new(),
        true,
    );

    (
        Arc::new(Mutex::new(Box::new(board_surround))),
        Arc::new(Mutex::new(Box::new(board_donut))),
    )
}

fn neighbor_benchmark_margin_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Small 1");

    for shape in SHAPES_1D_SMALL.iter() {
        let (surround, donut) = gen_board_1d(shape);
        group.bench_with_input(BenchmarkId::new("Surround", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = surround.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint1D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint1D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("Wrapping", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = donut.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint1D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint1D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, neighbor_benchmark_margin_1);
criterion_main!(benches);
