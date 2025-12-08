use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let boxes = parse_input(input)?;
    let mut distances = Vec::new();
    for (i, x) in boxes.iter().enumerate() {
        for (j, y) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((d2(x, y), i, j));
        }
    }
    distances.sort();

    let connections: HashMap<usize, Vec<usize>> =
        distances
            .iter()
            .take(1000)
            .fold(HashMap::new(), |mut acc, &(_, i, j)| {
                acc.entry(i).or_insert(Vec::new()).push(j);
                acc.entry(j).or_insert(Vec::new()).push(i);
                acc
            });

    let mut seen = HashSet::new();
    let mut components = Vec::new();
    for b in connections.keys() {
        if seen.contains(b) {
            continue;
        }
        let mut queue = vec![b];
        let mut component = HashSet::new();
        while let Some(n) = queue.pop() {
            for nn in connections.get(n).unwrap() {
                if !component.contains(nn) {
                    component.insert(nn);
                    seen.insert(nn);
                    queue.push(nn);
                }
            }
        }
        components.push(component);
    }

    let mut component_sizes: Vec<_> = components.iter().map(|c| c.len()).collect();
    component_sizes.sort_by(|a, b| b.cmp(a));

    Ok(component_sizes
        .iter()
        .take(3)
        .product::<usize>()
        .to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let boxes = parse_input(input)?;
    let mut distances = Vec::new();
    for (i, x) in boxes.iter().enumerate() {
        for (j, y) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((d2(x, y), i, j));
        }
    }
    distances.sort();

    let connections: HashMap<usize, Vec<usize>> =
        distances
            .iter()
            .fold(HashMap::new(), |mut acc, &(_, i, j)| {
                acc.entry(i).or_insert(Vec::new()).push(j);
                acc.entry(j).or_insert(Vec::new()).push(i);
                acc
            });

    println!("{}", connections.len());
    Err("Solution not implemented".into())
}

type Coord = [i64; 3];

fn parse_input(input: String) -> Result<Vec<Coord>, Box<dyn std::error::Error>> {
    let x = input
        .trim()
        .lines()
        .map(|line| {
            let values: Result<Vec<_>, _> = line.split(',').map(str::parse).collect();
            values.map(|v| [v[0], v[1], v[2]])
        })
        .collect::<Result<_, _>>()?;
    Ok(x)
}

fn d2(a: &Coord, b: &Coord) -> i64 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| (ai - bi).pow(2))
        .sum()
}
