use std::collections::HashMap;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (patterns, designs) = parse_input(input)?;

    Ok(designs
        .iter()
        .filter(|d| is_possible(d, &patterns))
        .count()
        .to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (patterns, designs) = parse_input(input)?;

    let mut counts_table: HashMap<String, usize> = HashMap::from_iter([("".to_string(), 0)]);
    Ok(designs
        .iter()
        .map(|d| count_ways(d, &patterns, &mut counts_table))
        .sum::<usize>()
        .to_string())
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

fn count_ways(design: &str, patterns: &[String], table: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = table.get(design) {
        return count;
    }

    let count = patterns
        .iter()
        .map(|p| {
            if design == p {
                1
            } else if design.starts_with(p) {
                count_ways(&design[p.len()..], patterns, table)
            } else {
                0
            }
        })
        .sum();
    table.insert(design.to_string(), count);
    count
}
