pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input
        .chars()
        .map(|p| match p {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum::<i64>()
        .to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut floor = 0;
    for (k, p) in input.chars().enumerate() {
        match p {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("!"),
        }
        if floor < 0 {
            return Ok((k + 1).to_string());
        }
    }
    Err("No solution found".into())
}
