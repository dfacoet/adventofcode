use core::panic;

use regex::Regex;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut mul_pairs: Vec<(u64, u64)> = Vec::new();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    for mat in re.captures_iter(&input) {
        if let (Some(x), Some(y)) = (mat.get(1), mat.get(2)) {
            if let (Ok(x), Ok(y)) = (x.as_str().parse(), y.as_str().parse()) {
                mul_pairs.push((x, y));
            }
        }
    }
    Ok(mul_pairs
        .iter()
        .map(|(x, y)| x * y)
        .sum::<u64>()
        .to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut mul_pairs: Vec<(u64, u64)> = Vec::new();
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?;
    for mat in re.captures_iter(&input) {
        match (mat.get(0).map(|m| m.as_str()), mat.get(1), mat.get(2)) {
            (Some(_), Some(x), Some(y)) => {
                if enabled {
                    if let (Ok(x), Ok(y)) = (x.as_str().parse(), y.as_str().parse()) {
                        mul_pairs.push((x, y));
                    }
                }
            }
            (Some("do()"), None, None) => enabled = true,
            (Some("don't()"), None, None) => enabled = false,
            _ => panic!("impossible"),
        }
    }
    Ok(mul_pairs
        .iter()
        .map(|(x, y)| x * y)
        .sum::<u64>()
        .to_string())
}
