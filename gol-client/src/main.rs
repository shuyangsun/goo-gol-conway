use clap::{App, Arg};
use gol_client::persistence::load_board::CellularAutomatonConfig;
use gol_core::predefined_states;
use rayon::prelude::*;
use serde_json;
use std::collections::HashMap;

fn main() {
    let jsons = vec![
        include_str!("../examples/tetris.json"),
        include_str!("../examples/random.json"),
        include_str!("../examples/random_gol.json"),
    ];
    let configs: Vec<CellularAutomatonConfig> = jsons
        .par_iter()
        .map(|ele| serde_json::from_str(ele).unwrap())
        .collect();

    let title_to_config: HashMap<String, CellularAutomatonConfig> = configs
        .into_par_iter()
        .map(|config| (config.title().clone(), config))
        .collect();

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let mut demos_description = String::from("Runs demo, available demos: ");
    let mut sorted_titiles: Vec<&String> = title_to_config.keys().collect();
    sorted_titiles.sort();
    for (i, title) in sorted_titiles.iter().enumerate() {
        demos_description.push_str(title);
        demos_description.push_str(if i < sorted_titiles.len() - 1 {
            ", "
        } else {
            "."
        });
    }

    let title_to_config: HashMap<String, CellularAutomatonConfig> = title_to_config
        .into_par_iter()
        .map(|(key, value)| (key.to_lowercase(), value))
        .collect();

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

    let is_donut = match matches.occurrences_of("donut") {
        0 => false,
        _ => true,
    };

    match matches.value_of("demo") {
        Some(demo_name) => title_to_config
            .get(&demo_name.to_lowercase())
            .unwrap()
            .run_board(),
        None => (),
    };
}
