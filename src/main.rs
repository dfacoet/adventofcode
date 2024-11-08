use std::{fs, io::Write};

use chrono::{Datelike, Utc};
use chrono_tz::America::New_York;
use clap::{Parser, Subcommand};

const MIN_YEAR: i32 = 2015;
const MAX_YEAR: i32 = 2023; // TODO: determine from date

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get puzzle input
    Get {
        // TODO: enforce either year+day or --all here
        /// Year (either specify a year and day, use --today or use --all)
        // #[arg(short, long, value_parser=value_parser!(u16).range(MIN_YEAR as i64..=MAX_YEAR as i64))]
        year: Option<i32>,
        /// Day
        // #[arg(short, long, value_parser=value_parser!(u8).range(1..=25))]
        day: Option<u32>,
        /// Get today's (EST) puzzle input
        #[arg(short, long)]
        today: bool,
        /// Get all puzzle inputs for the years available
        #[arg(long)]
        all: bool,
    },
    /// Run puzzle solutions
    Run {},
    /// Start an AoC day (get input, create template)
    Start {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get {
            year: Some(year),
            day: Some(day),
            today: false,
            all: false,
        } => write_input_file(year, day),
        Commands::Get {
            year: None,
            day: None,
            today: true,
            all: false,
        } => {
            let now_nyc = Utc::now().with_timezone(&New_York);
            if now_nyc.month() != 12 || now_nyc.day() > 25 {
                println!("It's not AoC time")
            } else {
                write_input_file(&now_nyc.year(), &now_nyc.day());
            }
        }
        Commands::Get {
            year: None,
            day: None,
            today: false,
            all: true,
        } => {
            // TODO:
            // - progress bar
            // - run asynchronously
            for year in MIN_YEAR..=MAX_YEAR {
                for day in 1..=25 {
                    write_input_file(&year, &day);
                }
            }
        }
        Commands::Get { .. } => println!("Must specify either year and day, or --all"),
        Commands::Run { .. } => println!("run not implemented yet"),
        Commands::Start { .. } => println!("start not implemented yet"),
    }
}

fn write_input_file(year: &i32, day: &u32) {
    let file_path_str = format!("input/y{year}d{:02}.txt", day);
    let file_path = std::path::Path::new(&file_path_str);

    if fs::create_dir(file_path.parent().unwrap()).is_ok() {
        println!("Directory created: {:?}", file_path.parent().unwrap());
    } // else: Directory exist, ok. Catch other errors?

    if file_path.exists() {
        println!("Input file {:?} already exists", file_path);
        return;
    }
    let input_text = match get_input(year, day) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not get input data, no files created");
            return;
        } // TODO: catch different errors
    };
    match fs::File::create_new(file_path) {
        Ok(mut file) => match file.write(input_text.as_bytes()) {
            Ok(_) => println!("File created {:?}", file_path),
            Err(e) => println!("Could not write to {:?}, {e}", file_path),
        },
        //TODO: optional overwrite
        Err(e) => println!("{:?}", e),
    }
}

const NOT_LOGGED_MESSAGE: &str =
    "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n";

const TOKEN_PATH_STR: &str = ".token";

fn get_input(year: &i32, day: &u32) -> Result<String, ()> {
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
