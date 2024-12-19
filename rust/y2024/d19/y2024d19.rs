pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (patterns, designs) = parse_input(input)?;

    Ok(designs
        .iter()
        .filter(|d| is_possible(d, &patterns))
        .count()
        .to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

fn parse_input(input: String) -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    let v = input.split("\n\n").collect::<Vec<_>>();
    let (patterns, designs) = if v.len() == 2 {
        (v[0], v[1])
    } else {
        return Err("Invalid input".into());
    };
    let patterns = patterns.split(", ").map(String::from).collect();
    let designs = designs.lines().map(String::from).collect();

    Ok((patterns, designs))
}

fn is_possible(design: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|p| {
        design == p || (design.starts_with(p) && is_possible(&design[p.len()..], patterns))
    })
}
