use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gol_core::{
    BinaryState, Board, ConwayStrategy, GridPoint1D, GridPoint2D, GridPoint3D, StandardBoardFactory,
};
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

const SHAPES_1D_SMALL: [usize; 14] = [
    5, 10, 20, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
];
const SHAPES_1D_LARGE: [usize; 11] = [
    5, 10_000, 20_000, 30_000, 40_000, 50_000, 60_000, 70_000, 80_000, 90_000, 100_000,
];

const SHAPES_2D_SMALL: [usize; 14] = [
    5, 10, 20, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
];
const SHAPES_2D_LARGE: [usize; 6] = [5, 1_000, 2_000, 3_000, 4_000, 5_000];

const SHAPES_3D_SMALL: [usize; 13] = [5, 10, 20, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900];

type ConwayBoard<CI> = Arc<Mutex<Box<dyn Board<BinaryState, CI, std::vec::IntoIter<CI>>>>>;

fn neighbor_benchmark_1d_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("1D Board Small");

    for shape in SHAPES_1D_SMALL.iter() {
        let (surround, donut) = gen_board_1d(shape, 1);
        group.bench_with_input(BenchmarkId::new("Surround, 1", shape), shape, |b, _| {
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
        group.bench_with_input(BenchmarkId::new("Wrapping, 1", shape), shape, |b, _| {
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
        let (surround, donut) = gen_board_1d(shape, 2);
        group.bench_with_input(BenchmarkId::new("Surround, 2", shape), shape, |b, _| {
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
        group.bench_with_input(BenchmarkId::new("Wrapping, 2", shape), shape, |b, _| {
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

fn neighbor_benchmark_1d_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("1D Board Large");

    for shape in SHAPES_1D_LARGE.iter() {
        let (surround, donut) = gen_board_1d(shape, 1);
        group.bench_with_input(BenchmarkId::new("Surround, 1", shape), shape, |b, _| {
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
        group.bench_with_input(BenchmarkId::new("Wrapping, 1", shape), shape, |b, _| {
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
        let (surround, donut) = gen_board_1d(shape, 2);
        group.bench_with_input(BenchmarkId::new("Surround, 2", shape), shape, |b, _| {
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
        group.bench_with_input(BenchmarkId::new("Wrapping, 2", shape), shape, |b, _| {
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

fn neighbor_benchmark_2d_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("2D Board Small");

    for shape in SHAPES_2D_SMALL.iter() {
        let (surround, donut) = gen_board_2d(shape, 1);
        group.bench_with_input(BenchmarkId::new("Surround, 1", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = surround.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("Wrapping, 1", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = donut.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        let (surround, donut) = gen_board_2d(shape, 2);
        group.bench_with_input(BenchmarkId::new("Surround, 2", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = surround.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("Wrapping, 2", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = donut.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
    }
    group.finish();
}

fn neighbor_benchmark_2d_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("2D Board Large");

    for shape in SHAPES_2D_LARGE.iter() {
        let (surround, donut) = gen_board_2d(shape, 1);
        group.bench_with_input(BenchmarkId::new("Surround, 1", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = surround.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("Wrapping, 1", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = donut.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        let (surround, donut) = gen_board_2d(shape, 2);
        group.bench_with_input(BenchmarkId::new("Surround, 2", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = surround.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("Wrapping, 2", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = donut.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint2D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint2D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
    }
    group.finish();
}

fn neighbor_benchmark_3d_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("3D Board Small");
    group.sample_size(20);

    for shape in SHAPES_3D_SMALL.iter() {
        let (surround, donut) = gen_board_3d(shape, 1);
        group.bench_with_input(BenchmarkId::new("Surround, 1", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = surround.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint3D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint3D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("Wrapping, 1", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = donut.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint3D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint3D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        let (surround, donut) = gen_board_3d(shape, 2);
        group.bench_with_input(BenchmarkId::new("Surround, 2", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = surround.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint3D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint3D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
        group.bench_with_input(BenchmarkId::new("Wrapping, 2", shape), shape, |b, _| {
            b.iter(|| {
                let unlocked = donut.lock().unwrap();
                let space_manager = unlocked.space_manager();
                let neighbor_manager = unlocked.neighbor_manager();
                let _: Vec<GridPoint3D<i32>> = space_manager
                    .indices_par_iter()
                    .map(|idx| {
                        let neighbors: Vec<GridPoint3D<i32>> =
                            neighbor_manager.get_neighbors_idx(&idx).collect();
                        neighbors.first().unwrap().clone()
                    })
                    .collect();
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    neighbor_benchmark_1d_small,
    neighbor_benchmark_1d_large,
    neighbor_benchmark_2d_small,
    neighbor_benchmark_2d_large,
    neighbor_benchmark_3d_small,
);
criterion_main!(benches);

fn gen_board_1d(
    shape: &usize,
    margin: usize,
) -> (ConwayBoard<GridPoint1D<i32>>, ConwayBoard<GridPoint1D<i32>>) {
    let strategy = Box::new(ConwayStrategy::new());
    let board_surround = StandardBoardFactory::new_binary_1d_grid(
        shape.clone(),
        BinaryState::Dead,
        BinaryState::Alive,
        margin,
        &HashSet::new(),
        strategy,
        Vec::new(),
        false,
    );

    let strategy = Box::new(ConwayStrategy::new());
    let board_donut = StandardBoardFactory::new_binary_1d_grid(
        shape.clone(),
        BinaryState::Dead,
        BinaryState::Alive,
        margin,
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

fn gen_board_2d(
    shape: &usize,
    margin: usize,
) -> (ConwayBoard<GridPoint2D<i32>>, ConwayBoard<GridPoint2D<i32>>) {
    let strategy = Box::new(ConwayStrategy::new());
    let board_surround = StandardBoardFactory::new_binary_2d_grid(
        (shape.clone(), shape.clone()),
        BinaryState::Dead,
        BinaryState::Alive,
        margin,
        &HashSet::new(),
        strategy,
        Vec::new(),
        false,
    );

    let strategy = Box::new(ConwayStrategy::new());
    let board_donut = StandardBoardFactory::new_binary_2d_grid(
        (shape.clone(), shape.clone()),
        BinaryState::Dead,
        BinaryState::Alive,
        margin,
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

fn gen_board_3d(
    shape: &usize,
    margin: usize,
) -> (ConwayBoard<GridPoint3D<i32>>, ConwayBoard<GridPoint3D<i32>>) {
    let strategy = Box::new(ConwayStrategy::new());
    let board_surround = StandardBoardFactory::new_binary_3d_grid(
        (shape.clone(), shape.clone(), shape.clone()),
        BinaryState::Dead,
        BinaryState::Alive,
        margin,
        &HashSet::new(),
        strategy,
        Vec::new(),
        false,
    );

    let strategy = Box::new(ConwayStrategy::new());
    let board_donut = StandardBoardFactory::new_binary_3d_grid(
        (shape.clone(), shape.clone(), shape.clone()),
        BinaryState::Dead,
        BinaryState::Alive,
        margin,
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
