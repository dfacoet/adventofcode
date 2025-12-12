use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (start, splitters) = parse_input(input)?;
    let mut beams = HashSet::from([start]);
    let mut count = 0;

    for row in splitters {
        beams = beams
            .iter()
            .flat_map(|x| {
                if row.contains(x) {
                    count += 1;
                    vec![x - 1, x + 1]
                } else {
                    vec![*x]
                }
            })
            .collect();
    }

    Ok(count.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (start, splitters) = parse_input(input)?;
    let mut paths = HashMap::from([(start, 1u64)]); // pos -> # paths

    for row in splitters {
        paths = paths.iter().fold(HashMap::new(), |mut acc, (&x, np)| {
            if row.contains(&x) {
                *acc.entry(x - 1).or_insert(0) += np;
                *acc.entry(x + 1).or_insert(0) += np;
            } else {
                *acc.entry(x).or_insert(0) += np;
            };
            acc
        })
    }

    Ok(paths.values().sum::<u64>().to_string())
}

type PositionsByRow = Vec<HashSet<usize>>;

fn parse_input(input: String) -> Result<(usize, PositionsByRow), Box<dyn std::error::Error>> {
    let mut lines = input.trim().lines();
    let start = lines
        .next()
        .ok_or("Invalid input")?
        .char_indices()
        .find_map(|(i, c)| if c == 'S' { Some(i) } else { None })
        .ok_or("No S found")?;
    let splitters = lines
        .map(|row| {
            row.char_indices()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect()
        })
        .collect();
    Ok((start, splitters))
}
