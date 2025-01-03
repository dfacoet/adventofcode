# adventofcode
Solutions to Advent of Code (AOC). Single repo for solutions in
- Python
- Rust
- Haskell

TODO: Migrate solutions scattered in other repos.

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
To start working on a puzzle, use
```
./aoc start [LANG] [YEAR] [DAY]
```

This downloads the puzzle input if necessary, and creates a template following the conventions of
the language subproject.

Supported languages: Python, Rust.
