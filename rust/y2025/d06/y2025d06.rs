use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let problems = parse_input1(input)?;
    let tot: u64 = problems.iter().map(|p| p.answer()).sum();
    Ok(tot.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let problems = parse_input2(input)?;
    let tot: u64 = problems.iter().map(|p| p.answer()).sum();
    Ok(tot.to_string())
}

fn parse_input1(input: String) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
    let ns = input
        .lines()
        .take_while(|s| s.chars().next().is_some_and(|c| c.is_digit(10)))
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let ops = input
        .trim()
        .lines()
        .last()
        .ok_or("Invalid input")?
        .split_whitespace()
        .map(|s| Op::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ops
        .into_iter()
        .enumerate()
        .map(|(i, op)| Problem {
            xs: ns.iter().map(|v| v[i]).collect(),
            op,
        })
        .collect())
}

fn parse_input2(input: String) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
    let rows: Vec<String> = input
        .lines()
        .map_while(|s| {
            if s.chars().next().is_some_and(|c| c.is_digit(10)) {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect();
    let ns = transpose(rows).fold(vec![vec![]], |mut groups, row| {
        match row.trim().parse::<u64>() {
            Ok(n) => groups.last_mut().unwrap().push(n),
            Err(_) => groups.push(Vec::new()),
        };
        groups
    });

    let ops = input
        .trim()
        .lines()
        .last()
        .ok_or("Invalid input")?
        .split_whitespace()
        .map(|s| Op::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ops
        .into_iter()
        .zip(ns.into_iter())
        .map(|(op, xs)| Problem { xs, op })
        .collect())
}

fn transpose(rows: Vec<String>) -> impl Iterator<Item = String> {
    let nrows = rows.len();
    let ncols = rows.first().map(|r| r.len()).unwrap_or(0);

    (0..ncols).map(move |col| {
        let mut out = String::with_capacity(nrows);
        for r in &rows {
            out.push(r.chars().nth(col).unwrap_or(' '));
        }
        out
    })
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err("Invalid operation".into()),
        }
    }
}

#[derive(Debug)]
struct Problem {
    xs: Vec<u64>,
    op: Op,
}

impl Problem {
    fn answer(&self) -> u64 {
        match self.op {
            Op::Add => self.xs.iter().sum(),
            Op::Mul => self.xs.iter().product(),
        }
    }
}
