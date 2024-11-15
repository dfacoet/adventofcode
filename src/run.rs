use std::fs;

use crate::{get_today_nyc, DayParams};

pub fn run(params: &DayParams) -> Result<(), Box<dyn std::error::Error>> {
    match params {
        DayParams {
            year: Some(year),
            day: Some(day),
            today: false,
            all: false,
        } => run_solution(year, day),
        DayParams {
            year: None,
            day: None,
            today: true,
            all: false,
        } => {
            if let Some((year, day)) = get_today_nyc() {
                run_solution(&year, &day)
            } else {
                Err("Today is not an advent day".into())
            }
        }
        _ => Err("Invalid parameters for start".into()),
    }
}

type SolutionFn = fn(String) -> String;

fn run_solution(year: &i32, day: &u32) -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(format!("input/y{year}d{:02}.txt", day))?;

    println!("year {year} day {:02}", day);
    println!("=====================");
    // find a reasonable way to get the right sol functions (macros?)
    let (part1, part2): (SolutionFn, SolutionFn) = match (year, day) {
        (2015, 1) => (y2015d01::part1, y2015d01::part2),
        (2020, 1) => (y2020d01::part1, y2020d01::part2),
        _ => panic!("Solution not found"),
    };

    let sol1 = part1(input.clone());
    println!("Part 1: {sol1}");
    let sol2 = part2(input.clone());
    println!("Part 2: {sol2}");
    Ok(())
}
