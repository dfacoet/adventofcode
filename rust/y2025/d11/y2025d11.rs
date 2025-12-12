use std::collections::HashMap;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let outputs = parse_input(input)?;
    Ok(n_paths(&outputs, "you", "out").to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let outputs = parse_input(input)?;
    let count = match (
        n_paths(&outputs, "fft", "dac"),
        n_paths(&outputs, "dac", "fft"),
    ) {
        (0, 0) => 0,
        (0, n) => n_paths(&outputs, "svr", "dac") * n * n_paths(&outputs, "fft", "out"),
        (n, 0) => n_paths(&outputs, "svr", "fft") * n * n_paths(&outputs, "dac", "out"),
        _ => return Err("loop".into()),
    };
    Ok(count.to_string())
}

fn parse_input(input: String) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    input
        .trim()
        .lines()
        .map(|s| {
            let (k, r) = s.split_once(": ").ok_or("Invalid input")?;
            let v = r.split(' ').map(|s| s.to_string()).collect();
            Ok((k.to_string(), v))
        })
        .collect()
}

fn n_paths(outputs: &HashMap<String, Vec<String>>, from: &str, to: &str) -> u64 {
    let mut counts = HashMap::from([(to.to_string(), 1)]);
    n_paths_rec(outputs, from, to, &mut counts)
}

fn n_paths_rec(
    outputs: &HashMap<String, Vec<String>>,
    from: &str,
    _to: &str,
    counts: &mut HashMap<String, u64>,
) -> u64 {
    match counts.get(from) {
        Some(d) => *d,
        None => {
            let count = match outputs.get(from) {
                Some(ns) => ns
                    .iter()
                    .map(|n| n_paths_rec(outputs, n, _to, counts))
                    .sum(),
                None => 0,
            };
            counts.insert(from.to_string(), count);
            count
        }
    }
}
