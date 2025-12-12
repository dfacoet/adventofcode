use std::cmp::max;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (ranges, ids) = parse_input(input)?;
    let n = ids
        .iter()
        .filter(|&id| ranges.iter().any(|range| in_range(id, range)))
        .count();
    Ok(n.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (mut ranges, _) = parse_input(input)?;
    ranges.sort();
    let acc = (0, ranges[0]);
    let (tot, last) = ranges.into_iter().fold(acc, |(tot, current), next| {
        if in_range(&next.0, &current) {
            // Next interval intersects the current -> merge them
            (tot, (current.0, max(next.1, current.1)))
        } else {
            (add(tot, current), next)
        }
    });
    Ok(add(tot, last).to_string())
}

type Range = (u64, u64);

fn parse_input(input: String) -> Result<(Vec<Range>, Vec<u64>), Box<dyn std::error::Error>> {
    let (ranges_str, ids_str) = input.trim().split_once("\n\n").ok_or("Invalid input")?;
    let ranges = ranges_str
        .lines()
        .map(|s| -> Result<_, Box<dyn std::error::Error>> {
            let (a, b) = s.split_once('-').ok_or("Invalid input")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect::<Result<_, _>>()?;

    let ids = ids_str
        .lines()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    Ok((ranges, ids))
}

fn in_range(n: &u64, (a, b): &Range) -> bool {
    n >= a && n <= b
}

fn add(n: u64, (a, b): Range) -> u64 {
    n + b - a + 1
}
