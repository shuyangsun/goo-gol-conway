fn main() {
    #[cfg(not(any(feature = "ascii")))]
    panic!("No render engine backend specified. Please re-compile with \"--features [ascii, gl, vulkan, metal, dx11, dx12]\" to set render engine.");

    #[cfg(any(feature = "ascii"))]
    {
        use clap::{App, Arg};
        use gol_client::demo;
        use gol_core::predefined_states;
        use std::collections::HashMap;

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
                Arg::with_name("gen_count")
                    .short("g")
                    .long("gen_count")
                    .value_name("GEN_COUNT")
                    .help("Specify the maximum number of simulation generation.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("delay")
                    .short("d")
                    .long("delay")
                    .value_name("DELAY")
                    .help("Sets initial delay in seconds before evolution starts.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("interval")
                    .short("i")
                    .long("interval")
                    .value_name("INTERVAL")
                    .help("Sets delay in seconds in-between generations.")
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

        let demo_name = matches.value_of("demo").unwrap();
        let max_iter: usize = matches
            .value_of("gen_count")
            .unwrap()
            .parse()
            .expect("Cannot parse max iteration count to integer.");
        let delay: f64 = matches
            .value_of("delay")
            .unwrap()
            .parse()
            .expect("Cannot parse initial delay seconds to float.");
        let interval: f64 = matches
            .value_of("interval")
            .unwrap()
            .parse()
            .expect("Cannot parse interval seconds to float.");
        let (initial_states, title) = demos.get(demo_name).unwrap();
        let is_donut = match matches.occurrences_of("donut") {
            0 => false,
            _ => true,
        };
        demo::two_dimensional::run_demo(initial_states, title, max_iter, delay, interval, is_donut);
    }
}
