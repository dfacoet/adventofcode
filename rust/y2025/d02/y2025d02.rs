pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let ranges = parse_input(input)?;
    let total: u64 = ranges
        .into_iter()
        .flat_map(|(a, b)| (a..=b).filter(is_invalid_1))
        .sum();
    Ok(total.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let ranges = parse_input(input)?;
    let total: u64 = ranges
        .into_iter()
        .flat_map(|(a, b)| (a..=b).filter(is_invalid_2))
        .sum();
    Ok(total.to_string())
}

fn parse_input(input: String) -> Result<Vec<(u64, u64)>, Box<dyn std::error::Error>> {
    input
        .trim()
        .split(',')
        .map(|r| {
            let (a, b) = r.split_once('-').ok_or("missing -")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect()
}

fn is_invalid_1(n: &u64) -> bool {
    // must have an even number of digits
    // and be a multiple of 10..01 = 10^(nd/2) + 1
    let nd = n.ilog10() + 1;
    nd.is_multiple_of(2) && n.is_multiple_of(10u64.pow(nd / 2) + 1)
}

fn is_invalid_2(n: &u64) -> bool {
    panic!("{n}")
}
