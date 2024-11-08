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
    let input_text = match get_input(year, day) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not get input data, no files created");
            return;
        } // TODO: catch different errors
    };
    match fs::File::create_new(&file_path) {
        Ok(mut file) => match file.write(input_text.as_bytes()) {
            Ok(_) => println!("File created {:?}", file_path),
            Err(e) => println!("Could not write to {:?}, {e}", file_path),
        },
        //TODO: optional overwrite
        Err(e) => println!("File already exists: {:?}, {e}", file_path),
    }
}

const NOT_LOGGED_MESSAGE: &str =
    "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n";

const TOKEN_PATH_STR: &str = ".token";

fn get_input(year: u16, day: u8) -> Result<String, ()> {
    let token = fs::read_to_string(TOKEN_PATH_STR)
        .expect("Failed to read token file")
        .trim()
        .to_string();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", token))
        .send()
        .expect("Failed to send request");

    if response.status().is_success() {
        match response.text() {
            Ok(data) => {
                if data == NOT_LOGGED_MESSAGE {
                    println!("{}", data);
                    Err(())
                } else {
                    Ok(data)
                }
            }
            Err(_) => Err(()),
        }
    } else {
        Err(())
    }
}
