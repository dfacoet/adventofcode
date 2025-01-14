use std::{fs, path::PathBuf, str::FromStr};

use crate::{get_today_nyc, Language, RunDayParams};
use std::process::Command;

pub fn run(params: &RunDayParams) -> Result<(), Box<dyn std::error::Error>> {
    match params {
        RunDayParams {
            language,
            year: Some(year),
            day: Some(day),
            today: false,
            input,
        } => match language {
            Language::Haskell => run_haskell_solution(year, day, input),
            Language::Python => run_python_solution(year, day, input),
            Language::Rust => run_rust_solution(year, day, input),
        },
        RunDayParams {
            language,
            year: None,
            day: None,
            today: true,
            input,
        } => {
            if let Some((year, day)) = get_today_nyc() {
                run(&RunDayParams {
                    language: *language,
                    year: Some(year),
                    day: Some(day),
                    today: false,
                    input: input.clone(),
                })
            } else {
                Err("Today is not an advent day".into())
            }
        }
        _ => Err("Invalid parameters for start".into()),
    }
}

type SolutionFn = fn(String) -> Result<String, Box<dyn std::error::Error>>;

fn run_rust_solution(
    year: &i32,
    day: &u32,
    input_path: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = match input_path {
        Some(path) => path.clone(),
        None => PathBuf::from_str(&format!("input/y{year}d{:02}.txt", day))?,
    };
    let input = fs::read_to_string(input_path)?;

    println!("year {year} day {:02}", day);
    println!("================");
    let (part1, part2) = get_solution_functions(year, day)?;

    let sol1 = part1(input.clone())?;
    println!("Part 1: {sol1}");
    let sol2 = part2(input.clone())?;
    println!("Part 2: {sol2}");
    Ok(())
}

fn get_solution_functions(
    year: &i32,
    day: &u32,
) -> Result<(SolutionFn, SolutionFn), Box<dyn std::error::Error>> {
    // find a reasonable way to get the right sol functions (macros?)
    match (year, day) {
        (2015, 1) => Ok((y2015d01::part1, y2015d01::part2)),
        (2018, 23) => Ok((y2018d23::part1, y2018d23::part2)),
        (2020, 1) => Ok((y2020d01::part1, y2020d01::part2)),
        (2023, 4) => Ok((y2023d04::part1, y2023d04::part2)),
        (2024, 3) => Ok((y2024d03::part1, y2024d03::part2)),
        (2024, 6) => Ok((y2024d06::part1, y2024d06::part2)),
        (2024, 8) => Ok((y2024d08::part1, y2024d08::part2)),
        (2024, 9) => Ok((y2024d09::part1, y2024d09::part2)),
        (2024, 10) => Ok((y2024d10::part1, y2024d10::part2)),
        (2024, 11) => Ok((y2024d11::part1, y2024d11::part2)),
        (2024, 12) => Ok((y2024d12::part1, y2024d12::part2)),
        (2024, 13) => Ok((y2024d13::part1, y2024d13::part2)),
        (2024, 14) => Ok((y2024d14::part1, y2024d14::part2)),
        (2024, 15) => Ok((y2024d15::part1, y2024d15::part2)),
        (2024, 16) => Ok((y2024d16::part1, y2024d16::part2)),
        (2024, 17) => Ok((y2024d17::part1, y2024d17::part2)),
        (2024, 18) => Ok((y2024d18::part1, y2024d18::part2)),
        (2024, 19) => Ok((y2024d19::part1, y2024d19::part2)),
        (2024, 20) => Ok((y2024d20::part1, y2024d20::part2)),
        (2024, 21) => Ok((y2024d21::part1, y2024d21::part2)),
        (2024, 24) => Ok((y2024d24::part1, y2024d24::part2)),
        _ => Err(format!("Solution code not found for {year}/{day}").into()),
    }
}

// TODO: unify run_solutions that just runs a Command?

fn run_python_solution(
    year: &i32,
    day: &u32,
    input_path: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("uv")
        .arg("run")
        .arg("python")
        .arg("-m")
        .arg("pyaoc")
        .arg(year.to_string())
        .arg(day.to_string())
        .args(match input_path {
            Some(path) => vec!["--input", path.to_str().unwrap()],
            None => vec![],
        })
        .status()?;

    if !status.success() {
        return Err("Failed to run Python solution".into());
    }

    Ok(())
}

fn run_haskell_solution(
    year: &i32,
    day: &u32,
    input_path: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("stack")
        .arg("exec")
        .arg("haskell-exe")
        .arg("--")
        .arg(year.to_string())
        .arg(day.to_string())
        .args(match input_path {
            Some(path) => vec!["--input", path.to_str().unwrap()],
            None => vec![],
        })
        .status()?;

    if !status.success() {
        return Err("Failed to run Haskell solution".into());
    }

    Ok(())
}
