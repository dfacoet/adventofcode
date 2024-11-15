# adventofcode
Advent of code (AOC) solutions single repo

## CLI
This project provides the `aoc` CLI utility.

### Setup
- To build the CLI, just run `make`. This requires the Rust toolchain. The `aoc` executable will be placed in the root directory.
- Puzzle inputs differ by user. To download them, the CLI needs to be authenticated. Get your session cookie (see [here](https://github.com/wimglenn/advent-of-code-wim/issues/1) for ideas on how to do it)
and save it to `./.token`.

### Usage
To download a specific day's puzzle input (e.g. the first day of 2015):
```bash
./aoc get 2015 1
```
This will download the input and save it to `./input/y2015d1.txt`.

To download all puzzle inputs:
```bash
./aoc get --all
```
The download is incremental: if a file is already present, it will not be downloaded again.

For full instructions,
```bash
./aoc -h
```

## Solutions
### Rust
Each solution is a crate library located at `rust/y<YEAR>/d<DAY>`. Use

```bash
./aoc start [YEAR] [DAY]
```
to generate a template. This also downloads the puzzle input if necessary, and updates the global Cargo.toml to integrate the solution.

Note: the match statement in `run::get_solution_functions` needs to be updated manually (TODO: automate). After that, run a solution with

```bash
./aoc run [YEAR] [DAY]
```

This will load the input, run the solution functions, and print the answers.
