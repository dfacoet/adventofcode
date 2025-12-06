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
    let mut lines = input.lines().rev();
    let ops = lines
        .next()
        .ok_or("Invalid input")?
        .split_whitespace()
        .map(Op::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let ns = lines
        .rev()
        .take_while(|s| s.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()
        })
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
    let mut lines = input.lines().rev();
    let ops = lines
        .next()
        .ok_or("Invalid input")?
        .split_whitespace()
        .map(Op::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let rows: Vec<String> = lines
        .rev()
        .map_while(|s| {
            if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect();
    let ns = transpose_string(rows).fold(vec![vec![]], |mut groups, row| {
        match row.trim().parse::<u64>() {
            Ok(n) => groups.last_mut().unwrap().push(n),
            Err(_) => groups.push(Vec::new()),
        };
        groups
    });

    Ok(ops
        .into_iter()
        .zip(ns)
        .map(|(op, xs)| Problem { xs, op })
        .collect())
}

fn transpose_string(rows: Vec<String>) -> impl Iterator<Item = String> {
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
