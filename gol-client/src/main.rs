use clap::{App, Arg};
use gol_client::{demo, persistence::load_board::CellularAutomatonConfig};
use gol_core::predefined_states;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};

fn main() {
    let tetris_json = include_str!("../examples/tetris.json");
    let config: CellularAutomatonConfig = serde_json::from_str(tetris_json).unwrap();
    config.run_board();

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let mut demos = HashMap::new();
    demos.insert("tetris", (predefined_states::conway_2d_tetris(), "Tetris"));
    demos.insert("glider", (predefined_states::conway_2d_glider(), "Glider"));
    demos.insert(
        "generator",
        (
            predefined_states::conway_2d_glider_gun(),
            "Glider Generator",
        ),
    );
    demos.insert(
        "eater",
        (predefined_states::conway_2d_eater(), "Glider Eater"),
    );
    demos.insert(
        "generator_and_eater",
        (
            predefined_states::conway_2d_glider_gun_with_eater(),
            "Glider Generator and Eater",
        ),
    );
    demos.insert(
        "and_gate_00",
        (predefined_states::conway_2d_and_gate_00(), "AND Gate 00"),
    );
    demos.insert(
        "and_gate_01",
        (predefined_states::conway_2d_and_gate_01(), "AND Gate 01"),
    );
    demos.insert(
        "and_gate_10",
        (predefined_states::conway_2d_and_gate_10(), "AND Gate 10"),
    );
    demos.insert(
        "and_gate_11",
        (predefined_states::conway_2d_and_gate_11(), "AND Gate 11"),
    );

    let mut demos_description = String::from("Runs demo, available demos: ");
    for (i, key_val) in demos.iter().enumerate() {
        let key = key_val.0;
        demos_description.push_str(key);
        demos_description.push_str(if i < demos.len() - 1 { ", " } else { "." });
    }

    let default_interval = 1.0;

    let matches = App::new("Game of Life on Steriods")
            .version(VERSION)
            .author("Shuyang Sun <shuyangsun10@gmail.com>")
            .about("A very generic implementation of cellular automatons.")
            .arg(
                Arg::with_name("demo")
                    .long("demo")
                    .value_name("DEMO_NAME")
                    .help(demos_description.as_str())
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("random")
                    .long("random")
                    .value_name("RANDOM_ALIVE_RATIO")
                    .help("Generates a random 2D Game of Life board, specify the ratio of alive cells with value between 0 and 1.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("gen_count")
                    .short("g")
                    .long("gen_count")
                    .value_name("GEN_COUNT")
                    .help("Specify the maximum number of simulation generation, if unspecififed the simulation will continue running until user itervention.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("interval")
                    .short("i")
                    .long("interval")
                    .value_name("INTERVAL") .help(format!("Sets delay in seconds in-between generations, default value {} second(s).", default_interval).as_str())
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("width")
                    .long("width")
                    .value_name("WINDOW_WIDTH")
                    .help("Optional value to set window width.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("height")
                    .long("height")
                    .value_name("WINDOW_HEIGHT")
                    .help("Optional value to set window height.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("donut")
                    .long("donut")
                    .value_name("IS_DONUT")
                    .help("Make your board a donut!")
                    .takes_value(false),
            )
            .get_matches();

    let max_iter: usize = match matches.value_of("gen_count") {
        Some(val) => val
            .parse()
            .expect("Cannot parse max iteration count to integer."),
        None => usize::MAX,
    };
    let interval: f64 = matches
        .value_of("interval")
        .unwrap_or(format!("{}", default_interval).as_str())
        .parse()
        .expect("Cannot parse interval seconds to float.");
    let alive_ratio: f64 = matches
        .value_of("random")
        .unwrap_or("0.0")
        .parse()
        .expect("Cannot parse alive ratio to float.");
    let width: Option<usize> = match matches.value_of("width") {
        Some(val) => Some(
            val.parse::<usize>()
                .expect("Height is not positive integer."),
        ),
        None => None,
    };
    let height: Option<usize> = match matches.value_of("height") {
        Some(val) => Some(
            val.parse::<usize>()
                .expect("Height is not positive integer."),
        ),
        None => None,
    };

    let random_states = (HashSet::new(), "Random Game of Life");
    let (initial_states, title) = match matches.value_of("demo") {
        Some(demo_name) => demos.get(demo_name).unwrap().clone(),
        None => random_states,
    };
    let is_donut = match matches.occurrences_of("donut") {
        0 => false,
        _ => true,
    };
    demo::two_dimensional::run_demo(
        width,
        height,
        initial_states,
        title,
        max_iter,
        interval,
        is_donut,
        alive_ratio,
    );
}
