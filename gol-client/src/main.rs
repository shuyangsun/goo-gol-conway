use clap::{App, Arg};
use gol_client::persistence::load_board::CellularAutomatonConfig;
use rayon::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs;

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

    let mut demos_description = String::from("Run demo, available demos: ");
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

    let matches = App::new("Game of Life on Steriods")
        .version(VERSION)
        .author("Shuyang Sun <shuyangsun10@gmail.com>")
        .about("A research-oriented generic implementation of cellular automaton.")
        .arg(
            Arg::with_name("demo")
                .long("demo")
                .value_name("NAME")
                .help(demos_description.as_str())
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .long("config")
                .value_name("FILE_PATH")
                .help("Path to config file.")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("demo") {
        Some(demo_name) => title_to_config
            .get(&demo_name.to_lowercase())
            .unwrap()
            .run_board(),
        None => (),
    };

    match matches.value_of("config") {
        Some(path) => {
            let content = fs::read_to_string(path).expect("Cannot read configuration file.");
            let config = CellularAutomatonConfig::from_json(content.as_str());
            config.run_board();
        }
        None => (),
    };
}
