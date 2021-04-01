use clap::{App, Arg};
use gol_client::persistence::load_board::CellularAutomatonConfig;
use rayon::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs;

fn main() {
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
        include_str!("../examples/star_wars.json"),
        include_str!("../examples/brians_brain.json"),
        include_str!("../examples/bombers.json"),
        include_str!("../examples/bombers_255.json"),
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

    let mut demos_description = String::from("Run demo, available demos are (case insensitive): ");
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
                .short("d")
                .long("demo")
                .value_name("NAME")
                .help(demos_description.as_str())
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help(
                    "Path to JSON configuration file.
Examples at https://github.com/shuyangsun/goo-gol-conway/tree/main/gol-client/examples",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .value_name("DIR")
                .help(
                    "Path to the directory to save run history for replay or analysis later.
If the directory does not exist it will be created,
if it does exist it must be empty, otherwise the program will terminate with error.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("replay")
                .short("r")
                .long("replay")
                .value_name("DIR")
                .help("Path to replay directory, use \"-t\" option to render the system with triangles.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("triangular")
                .short("t")
                .long("triangular")
                .value_name("IS_TRIANGULAR")
                .help("Convert the 2D square grid to triangular grid, only supported for configurations with non-extended Moore neighbor rule.")
                .takes_value(false),
        )
        .get_matches();

    let is_triangular = matches.is_present("triangular");

    match matches.value_of("replay") {
        Some(replay_path) => start_replay(&String::from(replay_path), is_triangular),
        None => (),
    };

    let save_dir = match matches.value_of("save") {
        Some(save_dir) => Some(String::from(save_dir)),
        None => None,
    };

    match matches.value_of("demo") {
        Some(demo_name) => {
            let board_config = title_to_config.get(&demo_name.to_lowercase()).unwrap();
            board_config.run_board(save_dir.clone(), is_triangular);
        }
        None => (),
    };

    match matches.value_of("config") {
        Some(path) => {
            let content = fs::read_to_string(path).expect("Cannot read configuration file.");
            let config = CellularAutomatonConfig::from_json(content.as_str());
            config.run_board(save_dir, is_triangular);
        }
        None => (),
    };
}

fn start_replay(local_path: &String, is_triangular: bool) {
    use gol_client::replay::replayer_local::ReplayerLocal;
    use gol_core::{util::grid_util::Shape2D, GridPoint2D};
    use gol_renderer::{
        renderer::keyboard_control::KeyboardControl, CellularAutomatonRenderer,
        DiscreteStateColorMap, GraphicalRendererGrid2D,
    };

    let control = KeyboardControl::new();
    let control_receiver = control.clone_receive_only();

    let replayer: ReplayerLocal<u8, GridPoint2D<i32>, (Shape2D, usize)> =
        ReplayerLocal::new(0, local_path).with_keyboard_control(control_receiver);
    let (board_shape, num_states) = replayer.get_header();

    let mut renderer = GraphicalRendererGrid2D::new(
        board_shape.width(),
        board_shape.height(),
        replayer.get_readonly_states(),
    )
    .ok()
    .unwrap()
    .with_keyboard_control(control);

    if is_triangular {
        renderer = renderer.with_triangles();
    }

    renderer.run(Box::new(DiscreteStateColorMap::new(num_states)));
}
