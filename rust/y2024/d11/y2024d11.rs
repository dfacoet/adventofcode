use std::collections::HashMap;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut stone_counts = parse_input(input);
    for _ in 0..25 {
        stone_counts = blink(&stone_counts)?;
    }
    Ok(stone_counts.values().sum::<usize>().to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut stone_counts = parse_input(input);
    for _ in 0..75 {
        stone_counts = blink(&stone_counts)?;
    }
    Ok(stone_counts.values().sum::<usize>().to_string())
}

fn parse_input(input: String) -> HashMap<usize, usize> {
    // Assumes no duplicates
    input
        .trim()
        .split(" ")
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn blink(
    stones: &HashMap<usize, usize>,
) -> Result<HashMap<usize, usize>, Box<dyn std::error::Error>> {
    let mut new_counts = HashMap::new();
    for (n, c) in stones.iter() {
        if *n == 0 {
            *new_counts.entry(1).or_insert(0) += c;
        } else {
            let n_str = n.to_string();
            if n_str.len() % 2 == 0 {
                let (n1, n2) = n_str.split_at(n_str.len() / 2);
                *new_counts.entry(n1.parse()?).or_insert(0) += c;
                *new_counts.entry(n2.parse()?).or_insert(0) += c;
            } else {
                *new_counts.entry(n * 2024).or_insert(0) += c;
            }
        }
    }
    Ok(new_counts)
}
