pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    solve(input, 2)
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    solve(input, 12)
}

fn solve(input: String, n: usize) -> Result<String, Box<dyn std::error::Error>> {
    let result: Result<u64, Box<dyn std::error::Error>> = input
        .trim()
        .lines()
        .map(|r| max_joltage_n(r, n))
        .collect::<Result<Vec<_>, _>>()
        .map(|vec| vec.iter().sum());
    Ok(result?.to_string())
}

fn max_joltage_n(row: &str, n: usize) -> Result<u64, Box<dyn std::error::Error>> {
    if n == 1 {
        return row
            .chars()
            .max()
            .and_then(|d| d.to_digit(10))
            .map(|d| d as u64)
            .ok_or("Invalid bank".into());
    }
    let (i, first_digit) = row[..row.len() - (n - 1)]
        .char_indices()
        .rev()
        .max_by_key(|(_, c)| *c)
        .ok_or("Bank is too small")?;

    // Recursive is slow for long banks, consider iterating over the row only once.
    let result = 10u64.pow((n - 1) as u32) * first_digit.to_digit(10).ok_or("Invalid bank")? as u64
        + max_joltage_n(&row[i + 1..], n - 1)?;
    Ok(result)
}
