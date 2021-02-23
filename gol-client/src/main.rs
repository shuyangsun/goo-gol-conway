use clap::{App, Arg};
use gol_client::persistence::load_board::CellularAutomatonConfig;
use rayon::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs;

fn main() {
    {
        use gol_client::persistence::batch_deserializer_local::BatchDeserializerLocal;
        use gol_core::{util::grid_util::Shape2D, GridPoint2D, IndexedDataOwned};

        let deserializer: BatchDeserializerLocal<
            Shape2D,
            Vec<IndexedDataOwned<GridPoint2D<i32>, u8>>,
        > = BatchDeserializerLocal::new(&String::from("~/Desktop/ca_tests/history/tetris"));
        for i in 0usize..999999 {
            match deserializer.get(i) {
                Some(val) => {
                    let (header, content) = val;
                    let mut alive_count = 0usize;
                    for ele in &content.1 {
                        if ele.1 > 0 {
                            alive_count += 1;
                        }
                    }
                    println!(
                        "({}, {}), {}: {}",
                        header.width(),
                        header.height(),
                        content.0,
                        alive_count
                    );
                }
                None => break,
            }
        }
    }

    let mut jsons = vec![
        include_str!("../examples/tetris.json"),
        include_str!("../examples/glider.json"),
        include_str!("../examples/glider_gun.json"),
        include_str!("../examples/glider_eater.json"),
        include_str!("../examples/glider_gun_with_eater.json"),
        include_str!("../examples/and_gate_00.json"),
        include_str!("../examples/and_gate_01.json"),
        include_str!("../examples/and_gate_10.json"),
        include_str!("../examples/and_gate_11.json"),
        include_str!("../examples/random.json"),
        include_str!("../examples/random_gol.json"),
        include_str!("../examples/and_gate_4_neighbors.json"),
        include_str!("../examples/byte_state.json"),
        include_str!("../examples/star_wars.json"),
        include_str!("../examples/brians_brain.json"),
        include_str!("../examples/bombers.json"),
        include_str!("../examples/sedimental.json"),
    ];

    #[cfg(feature = "ascii")]
    jsons.push(include_str!("../examples/tetris_ascii.json"));

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
