use std::str::FromStr;

use good_lp::{constraint, default_solver, Expression, Solution, SolverModel};
use itertools::Itertools;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let machines = parse_input(input)?;
    let tot: usize = machines
        .iter()
        .map(|m| m.find_light_presses())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    Ok(tot.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let machines = parse_input(input)?;
    let tot: u64 = machines
        .iter()
        .map(|m| m.find_joltage_presses())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();
    Ok(tot.to_string())
}

fn parse_input(input: String) -> Result<Vec<Machine>, Box<dyn std::error::Error>> {
    input.trim().lines().map(Machine::from_str).collect()
}

#[derive(Debug)]
struct Machine {
    target: u16,       // binary representation gives on/off lights
    buttons: Vec<u16>, // i-th binary digit is 1 iff the button affects the i-th light
    joltage: Vec<u32>,
}

impl FromStr for Machine {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_str, rest) = s.split_once(" (").ok_or("Invalid input")?;
        let (button_str, joltage_str) = rest.split_once(" {").ok_or("Invalid input")?;
        let target = target_str
            .trim_start_matches('[')
            .trim_end_matches(']')
            .char_indices()
            .filter_map(|(i, c)| {
                if c == '#' {
                    Some(2u16.pow(i as u32))
                } else {
                    None
                }
            })
            .sum();
        let buttons = button_str
            .split(' ')
            .map(|s| -> Result<u16, Self::Err> {
                let ids = s
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ids.into_iter().map(|d| 2u16.pow(d)).sum())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let joltage = joltage_str
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            target,
            buttons,
            joltage,
        })
    }
}

impl Machine {
    fn find_light_presses(&self) -> Result<usize, Box<dyn std::error::Error>> {
        // Each button is either pressed or not
        // Iterate over combinations of buttons of increasing length, until one matches the target
        for n in 1..=self.buttons.len() {
            for bs in self.buttons.iter().combinations(n) {
                // xor all pressed buttons
                let s = bs.iter().fold(0u16, |acc, &b| acc ^ b);
                if s == self.target {
                    return Ok(n);
                }
            }
        }

        Err("Solution not found".into())
    }

    fn find_joltage_presses(&self) -> Result<u64, Box<dyn std::error::Error>> {
        good_lp::variables! {vars: 0 <= xs[self.buttons.len()] (integer); }

        let objective = xs.iter().sum::<Expression>();

        let constraints: Vec<_> = self
            .joltage
            .iter()
            .enumerate()
            .map(|(i, jolt)| {
                good_lp::constraint!(
                    self.buttons
                        .iter()
                        .zip(xs.iter())
                        .filter_map(|(b, x)| if b & (1 << i) != 0 { Some(*x) } else { None })
                        .sum::<Expression>()
                        == *jolt
                )
            })
            .collect();

        let problem = constraints.into_iter().fold(
            vars.minimise(&objective).using(default_solver), // unconstrained problem, fold to add all constraints
            |p, c| p.with(c),
        );
        let solution = problem.solve()?;

        Ok(solution.eval(objective) as u64)
    }
}
