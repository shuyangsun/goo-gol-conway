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
        demos.insert("tetris", predefined_states::conway_2d_tetris());
        demos.insert("glider", predefined_states::conway_2d_glider());
        demos.insert("generator", predefined_states::conway_2d_glider_gun());
        demos.insert(
            "generator_and_eater",
            predefined_states::conway_2d_glider_gun_with_eater(),
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
                Arg::with_name("iter")
                    .short("i")
                    .long("iter")
                    .value_name("MAX_ITER")
                    .help("Specify the maximum number of simulation generation.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("initial_delay")
                    .short("d")
                    .long("delay")
                    .value_name("INITIAL_DELAY")
                    .help("Sets initial delay in seconds before evolution starts.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("interval")
                    .short("g")
                    .long("interval")
                    .value_name("INTERVAL")
                    .help("Sets initial delay in seconds before evolution starts.")
                    .takes_value(true),
            )
            .get_matches();

        let demo_name = matches.value_of("demo").unwrap();
        let max_iter: usize = matches
            .value_of("iter")
            .unwrap()
            .parse()
            .expect("Cannot parse max iteration count to integer.");
        let delay: f64 = matches
            .value_of("initial_delay")
            .unwrap()
            .parse()
            .expect("Cannot parse initial delay seconds to float.");
        let interval: f64 = matches
            .value_of("interval")
            .unwrap()
            .parse()
            .expect("Cannot parse interval seconds to float.");
        demo::two_dimensional::run_demo(demos.get(demo_name).unwrap(), max_iter, delay, interval);
    }
}
