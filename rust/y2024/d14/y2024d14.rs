use std::collections::HashMap;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
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

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut robots: Vec<_> = input.lines().map(parse_line).collect();
    let mut min_dist = usize::MAX;
    let mut solution = None;
    for t in 0..10_000 {
        // TODO: max using lcm
        let sqdst: usize = robots
            .iter()
            .flat_map(|(p1, _)| {
                robots
                    .iter()
                    .map(|(p2, _)| (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2))
            })
            .sum();
        if sqdst < min_dist {
            min_dist = sqdst;
            solution = Some(t);
        }
        // TODO: modify robots in place
        robots = robots
            .iter()
            .map(|(p, v)| (evolve(*p, *v, 1), *v))
            .collect();
    }
    solution
        .map(|t| t.to_string())
        .ok_or("Solution not found".into())
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
