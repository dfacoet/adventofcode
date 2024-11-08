use std::{fs, io::Write};

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    command: String,
    year: u16,
    day: u8,
}

fn main() {
    let args = Cli::parse();

    match args.command.as_str() {
        "get" => write_input_file(args.year, args.day),
        c => panic!("Invalid command: {c}"), // TODO: add to parsing
    }
}

fn write_input_file(year: u16, day: u8) {
    let file_path_str = format!("input/y{year}d{day}.txt");
    let file_path = std::path::Path::new(&file_path_str);

    match fs::create_dir(file_path.parent().unwrap()) {
        Ok(_) => println!("Directory created: {:?}", file_path.parent().unwrap()),
        Err(_) => (), // Directory exist, ok. Catch other errors?
    }
    match fs::File::create_new(&file_path) {
        Ok(mut file) => {
            let input_text = get_input(year, day);
            match file.write(input_text.as_bytes()) {
                Ok(_) => println!("File created {:?}", file_path),
                Err(e) => println!("Could not write to {:?}, {e}", file_path),
            }
        }
        //TODO: optional overwrite
        Err(e) => println!("File already exists: {:?}, {e}", file_path),
    }
}

fn get_input(year: u16, day: u8) -> String {
    // TODO: download from aoc website
    format!("mock input test for {}/{}", year, day)
}
