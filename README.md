# Goo-GoL-Conway

A highly configurable Cellular Automaton program (John Conway's Game of Life on steroids).

Sadly Conway passed away on April 11, 2020 due to COVID-19. This project is to pay respects to him for his contribution to mathematics, [Google Conway](https://www.google.com/search?q=john+conway) to learn more about him.

## Installation

This program has been (not rigorously) tested on some Linux, macOS and Windows platforms, the following steps should work for all three platforms (with the exception of no ASCII rendering support on Windows).

Dedicated GPU hardware is preferred for faster rendering but not required. 

### Install Rust

If you do not already have Rust installed on your system, please follow the official Rust [installation documentation](https://www.rust-lang.org/tools/install) to install it.

It is good practice to run `rustup update` to make sure your Rust toolchain is up to date before installing this program. From here, you can choose to either do a simple installation using Cargo (recomended) or build from source.

### Using Cargo

```bash
$ cargo install gol-client
```

### Build from Source

```bash
# Clone respository
$ git clone git@github.com:shuyangsun/goo-gol-conway.git

# Change working directory
$ cd goo-gol-conway

# Update crates index
$ cargo update

# Build
$ cargo build --release

# Check instruction
$ ./target/release/gol --help
```

#### ASCII Rendering

In almost all use cases graphical rendering is desired, however sometimes limitations of the system may not allow a graphical representation (e.g., remote into a Linux terminal). In these situations you can still display grid-based two-dimensional Cellular Automaton systems with ASCII characters utilizing [NCURSES](https://tldp.org/HOWTO/NCURSES-Programming-HOWTO/) library. This library is only available on UNIX platforms, that is why it is not enabled by default.

To enable support for ASCII rendering, you need to install NCURSES first, and since the installation step is platform-dependent, we will not cover that in this document. After installing NCURSES, simply build the program with command `cargo build --features ascii --release`.

## Usage

```bash
# Check command-line argument options (and available demos).
$ gol --help

# Run a demo.
$ gol --demo starwars

# Run a demo, but in triangular mode.
$ gol --demo bombers --triangle

# Run from a JSON configuration file, checkout help message for examples.
$ gol --config /path/to/config.json

# Saving run result.
$ gol --config /path/to/config.json --save /path/to/empty/directory

# Replay from saved data.
$ gol --replay /path/to/replay/directory

# Replay using triangular rendering
$ gol --replay /path/to/replay/directory --triangle
```

### Playback Control Options

| Input | Functionality | Live | Replay |
| - | - | - | - |
| Scrolling | Zoom | Yes | Yes |
| Mouse Drag | Pan | Yes | Yes |
| Double Click | Reset Canvas | Yes | Yes |
| `SPACE` | Play/Pause | Yes | Yes |
| `J` | 0.5x Slow Down | Yes | Yes |
| `K` | 2x Speed Up | Yes | Yes |
| `H` (playing) | Play Backward | No | Yes |
| `L` (playing) | Play Forward | No | Yes |
| `H` (paused) | One Step Backwad | No | Yes |
| `L` (paused) | One Step Forward | No | Yes |