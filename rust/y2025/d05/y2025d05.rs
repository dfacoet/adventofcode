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
    let mut ranges = ranges.into_iter();
    let mut tot = 0;
    let mut current_range = ranges.next().ok_or("No ranges")?;

    for (a, b) in ranges {
        if in_range(&a, &current_range) {
            current_range.1 = max(current_range.1, b);
        } else {
            tot = add(tot, current_range);
            current_range = (a, b);
        }
    }
    tot = add(tot, current_range);
    Ok(tot.to_string())
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
