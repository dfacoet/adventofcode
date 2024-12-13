use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .split("\n\n")
        .map(Problem::from_str)
        .filter_map(|res| res.ok().and_then(|p| solver(&p, 0)))
        .sum::<u64>()
        .to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .split("\n\n")
        .map(Problem::from_str)
        .filter_map(|res| res.ok().and_then(|p| solver(&p, 10000000000000)))
        .sum::<u64>()
        .to_string())
}

#[derive(Debug)]
struct Problem {
    da: (u64, u64),
    db: (u64, u64),
    target: (u64, u64),
}

impl std::str::FromStr for Problem {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let parse_coords = |line: &str| -> Result<(u64, u64), Box<dyn std::error::Error>> {
            let parts: Vec<&str> = line.split(&[' ', ',', '+', '=']).collect();
            let x = parts.iter().position(|&s| s == "X").ok_or("Missing X")?;
            let y = parts.iter().position(|&s| s == "Y").ok_or("Missing Y")?;
            Ok((parts[x + 1].parse()?, parts[y + 1].parse()?))
        };

        Ok(Problem {
            da: parse_coords(lines.next().ok_or("Missing Button A")?)?,
            db: parse_coords(lines.next().ok_or("Missing Button B")?)?,
            target: parse_coords(lines.next().ok_or("Missing Prize")?)?,
        })
    }
}

fn solver(p: &Problem, shift: u64) -> Option<u64> {
    let ((d1, d2), (c1, c2)) = if slope(p.da) < slope(p.db) {
        ((p.da, p.db), (3, 1))
    } else {
        ((p.db, p.da), (1, 3))
    };
    let target = (p.target.0 + shift, p.target.1 + shift);
    if slope(d1) > slope(target) || slope(target) > slope(d2) {
        return None;
    }
    let num1 = d2.1 * target.0 - d2.0 * target.1;
    let num2 = d1.0 * target.1 - d1.1 * target.0;
    let den = d2.1 * d1.0 - d2.0 * d1.1;

    if num1 % den == 0 && num2 % den == 0 {
        Some(c1 * num1 / den + c2 * num2 / den)
    } else {
        None
    }
}

fn slope((i, j): (u64, u64)) -> f32 {
    j as f32 / i as f32
}
