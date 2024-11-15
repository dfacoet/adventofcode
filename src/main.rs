use std::{fs, io::Write};

use chrono::{Datelike, Utc};
use chrono_tz::America::New_York;
use clap::{Parser, Subcommand};
use regex::Regex;
mod run;

const MIN_YEAR: i32 = 2015;
const MAX_YEAR: i32 = 2023; // TODO: determine from date

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
struct DayParams {
    // TODO: enforce either year+day or --all here
    /// Year (either specify a year and day, use --today or use --all)
    // #[arg(short, long, value_parser=value_parser!(u16).range(MIN_YEAR as i64..=MAX_YEAR as i64))]
    year: Option<i32>,
    /// Day
    // #[arg(short, long, value_parser=value_parser!(u8).range(1..=25))]
    day: Option<u32>,
    /// Today's date (only valid during advent)
    #[arg(short, long)]
    today: bool,
    /// All days
    #[arg(long)]
    all: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Get puzzle input
    Get(DayParams),
    /// Run puzzle solutions
    Run(DayParams),
    /// Start an AoC day (get input, create template)
    // TODO: use different parameters, enable multiple languages
    Start(DayParams),
    /// Test the solutions
    Test(DayParams),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get(params) => get(params),
        Commands::Run(params) => run::run(params).unwrap(),
        Commands::Start(params) => start(params).unwrap(),
        Commands::Test(params) => test(params),
    }
}

fn get(params: &DayParams) {
    match params {
        DayParams {
            year: Some(year),
            day: Some(day),
            today: false,
            all: false,
        } => write_input_file(year, day),
        DayParams {
            year: None,
            day: None,
            today: true,
            all: false,
        } => {
            if let Some((year, day)) = get_today_nyc() {
                write_input_file(&year, &day);
            } else {
                // Error out? Use result/err instead of option?
                println!("Today is not an advent day");
            }
        }
        DayParams {
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
        _ => panic!("invalid options for get"),
    }
}

fn get_today_nyc() -> Option<(i32, u32)> {
    let now_nyc = Utc::now().with_timezone(&New_York);
    if now_nyc.month() != 12 || now_nyc.day() > 25 {
        None
    } else {
        Some((now_nyc.year(), now_nyc.day()))
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

fn get_url<U>(url: U) -> reqwest::blocking::Response
where
    U: reqwest::IntoUrl,
{
    let token = fs::read_to_string(TOKEN_PATH_STR)
        .expect("Failed to read token file")
        .trim()
        .to_string();

    let client = reqwest::blocking::Client::new();

    client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={}", token))
        .send()
        .expect("Failed to send request")
}

fn get_input(year: &i32, day: &u32) -> Result<String, ()> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let response = get_url(url);

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

fn test(params: &DayParams) {
    println!("Note: test only gets the answers for solved puzzles");
    match params {
        DayParams {
            year: Some(year),
            day: Some(day),
            today: false,
            all: false,
        } => {
            write_input_file(year, day);
            write_answer_file(year, day)
        }
        DayParams {
            year: None,
            day: None,
            today: true,
            all: false,
        } => {
            if let Some((year, day)) = get_today_nyc() {
                write_input_file(&year, &day);
                write_answer_file(&year, &day);
            } else {
                // Error out? Use result/err instead of option?
                println!("Today is not an advent day");
            }
        }
        DayParams {
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
                    write_answer_file(&year, &day);
                }
            }
        }
        _ => panic!("invalid options for test"),
    }
}

fn write_answer_file(year: &i32, day: &u32) {
    //TODO: refactor, have single write_file function etc
    let file_path_str = format!("answers/y{year}d{:02}.txt", day);
    let file_path = std::path::Path::new(&file_path_str);

    if fs::create_dir(file_path.parent().unwrap()).is_ok() {
        println!("Directory created: {:?}", file_path.parent().unwrap());
    } // else: Directory exist, ok. Catch other errors?

    if file_path.exists() {
        println!("Answer file {:?} already exists", file_path);
        return;
    }
    let answer_text = match get_answer(year, day) {
        Ok(data) => data,
        Err(_) => {
            println!("Could not get answer data, no files created");
            return;
        } // TODO: catch different errors
    };
    match fs::File::create_new(file_path) {
        Ok(mut file) => match file.write(answer_text.as_bytes()) {
            Ok(_) => println!("File created {:?}", file_path),
            Err(e) => println!("Could not write to {:?}, {e}", file_path),
        },
        //TODO: optional overwrite
        Err(e) => println!("{:?}", e),
    }
}

fn get_answer(year: &i32, day: &u32) -> Result<String, ()> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");

    let response = get_url(url);

    if response.status().is_success() {
        match response.text() {
            Ok(page) => parse_answers_page(page),
            Err(_) => Err(()),
        }
    } else {
        Err(())
    }
}

fn parse_answers_page(page: String) -> Result<String, ()> {
    let re = Regex::new(r#"Your puzzle answer was <code>(.*?)</code>."#).unwrap();
    let mut answers = String::new();

    for cap in re.captures_iter(&page) {
        answers.push_str(&cap[1]);
        answers.push('\n');
    }

    if answers.is_empty() {
        Err(())
    } else {
        Ok(answers)
    }
}

fn start(params: &DayParams) -> Result<(), Box<dyn std::error::Error>> {
    match params {
        DayParams {
            year: Some(year),
            day: Some(day),
            today: false,
            all: false,
        } => create_template(year, day),
        DayParams {
            year: None,
            day: None,
            today: true,
            all: false,
        } => {
            if let Some((year, day)) = get_today_nyc() {
                create_template(&year, &day)
            } else {
                Err("Today is not an advent day".into())
            }
        }
        _ => Err("Invalid parameters for start".into()),
    }
}

const RUST_TEMPLATE: &str =
    "pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 1
    Err(\"Solution not implemented\".into())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err(\"Solution not implemented\".into())
}
";

fn day_cargo_template(year: &i32, day: &u32) -> String {
    let name = format!("y{year}d{:02}", day);
    format!(
        "[package]
name = \"{name}\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
path = \"{name}.rs\"

[dependencies]
aoc = \"*\"
"
    )
}

fn create_template(year: &i32, day: &u32) -> Result<(), Box<dyn std::error::Error>> {
    write_input_file(year, day); // TODO: check silently

    // create daily project directory and template source and Cargo.toml files
    let new_project_path_str = format!("rust/y{year}/d{:02}", day);
    let new_project_path = std::path::Path::new(&new_project_path_str);
    fs::create_dir_all(new_project_path)?;

    let solution_path = new_project_path.join(format!("y{year}d{:02}.rs", day));
    match fs::File::create_new(&solution_path) {
        Ok(mut file) => file.write_all(RUST_TEMPLATE.as_bytes()).unwrap(),
        Err(e) => return Err(Box::new(e)),
    }
    let day_toml_path = new_project_path.join("Cargo.toml");
    match fs::File::create_new(&day_toml_path) {
        Ok(mut file) => file
            .write_all(day_cargo_template(year, day).as_bytes())
            .unwrap(),
        Err(e) => return Err(Box::new(e)),
    }

    // update global Cargo.toml
    let cargo_toml_content = fs::read_to_string("./Cargo.toml")?;
    let mut doc: toml_edit::DocumentMut = cargo_toml_content.parse()?;
    let members = doc["workspace"]["members"].as_array_mut().unwrap();

    if !members
        .iter()
        .any(|m| m.as_str() == Some(&new_project_path_str))
    {
        members.push(new_project_path_str.as_str());
    }
    members.sort_by(|a, b| a.as_str().unwrap().cmp(b.as_str().unwrap()));

    let dependencies = doc["dependencies"].as_table_mut().unwrap();
    let dependency_name = format!("y{year}d{:02}", day);
    dependencies.insert(
        &dependency_name,
        toml_edit::value({
            let mut dep = toml_edit::InlineTable::new();
            dep.insert(
                "path",
                toml_edit::value(new_project_path_str.as_str())
                    .into_value()
                    .unwrap(),
            );
            dep
        }),
    );

    fs::write("Cargo.toml", doc.to_string())?;

    Ok(())
}
