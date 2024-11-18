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
        (2020, 1) => Ok((y2020d01::part1, y2020d01::part2)),
        (2023, 4) => Ok((y2023d04::part1, y2023d04::part2)),
        _ => Err(format!("Solution code not found for {year}/{day}").into()),
    }
}

// TODO: unify run_solutions that just runs a Command?

fn run_python_solution(
    year: &i32,
    day: &u32,
    input_path: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("uv run python -m pyaoc")
        .arg(year.to_string())
        .arg(day.to_string())
        .args(
            input_path // add --input path if input_path is Some(path)
                .as_ref()
                .map(|path| vec!["--input", path.to_str().unwrap()])
                .unwrap_or_default(),
        )
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
    let status = Command::new("stack exec haskell-exe")
        .arg(year.to_string())
        .arg(day.to_string())
        .args(
            input_path // add --input path if input_path is Some(path)
                .as_ref()
                .map(|path| vec!["--input", path.to_str().unwrap()])
                .unwrap_or_default(),
        )
        .status()?;

    if !status.success() {
        return Err("Failed to run Python solution".into());
    }

    Ok(())
}
