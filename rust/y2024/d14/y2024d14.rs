use std::collections::HashMap;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .trim()
        .lines()
        .map(parse_line)
        .map(|(p, v)| evolve(p, v, 100))
        .map(quadrant)
        .fold(HashMap::new(), |mut counts, q| {
            *counts.entry(q).or_insert(0) += 1;
            counts
        })
        .iter()
        .filter_map(|(k, v)| k.map(|_| v))
        .product::<u64>()
        .to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

fn parse_line(line: &str) -> ((usize, usize), (i64, i64)) {
    let parts: Vec<&str> = line.split(" ").collect();
    let p: Vec<_> = parts[0][2..]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    let v: Vec<i64> = parts[1][2..]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    ((p[0], p[1]), (v[0], v[1]))
}

fn evolve(p: (usize, usize), v: (i64, i64), n: usize) -> (usize, usize) {
    (
        ((p.0 as i64 + n as i64 * v.0).rem_euclid(101) as usize),
        ((p.1 as i64 + n as i64 * v.1).rem_euclid(103) as usize),
    )
}

fn quadrant(p: (usize, usize)) -> Option<u8> {
    if p.0 == 50 || p.1 == 51 {
        None
    } else {
        Some((2 * (p.0 / 51) + p.1 / 52) as u8)
    }
}
