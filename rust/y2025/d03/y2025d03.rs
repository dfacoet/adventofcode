pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let result: Result<u32, Box<dyn std::error::Error>> = input
        .trim()
        .lines()
        .map(max_joltage)
        .collect::<Result<Vec<_>, _>>()
        .map(|vec| vec.iter().sum());
    Ok(result?.to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

fn max_joltage(row: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let first_digit = row[..row.len() - 1].chars().max().ok_or("Bank is empty")?;
    let (i, _) = row
        .chars()
        .enumerate()
        .find(|(_, c)| *c == first_digit)
        .unwrap();
    // TODO: do it in one go with something like row.chars().enumerate().max_by_key
    // but max methods return the last value
    let second_digit = row[i + 1..].chars().max().ok_or("Bank is empty")?;
    let result = 10 * first_digit.to_digit(10).ok_or("Invalid bank")?
        + second_digit.to_digit(10).ok_or("Invalid bank")?;
    Ok(result)
}
