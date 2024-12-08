use std::collections::{HashMap, HashSet};

type Coord = (i64, i64);

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (antennas, grid_shape) = parse_input(input);

    let antinodes: HashSet<(i64, i64)> = antennas
        .values()
        .flat_map(get_antinodes)
        .filter(|(x, y)| *x >= 0 && *x < grid_shape.0 && *y >= 0 && *y < grid_shape.1)
        .collect();

    Ok(antinodes.len().to_string())
}

// 302 is too high

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

fn parse_input(input: String) -> (HashMap<char, Vec<Coord>>, Coord) {
    let mut antennas = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.char_indices() {
            match c {
                '.' => continue,
                _ => antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push((i as i64, j as i64)),
            }
        }
    }
    let grid_shape = (
        input.lines().count() as i64,
        input.lines().next().unwrap().len() as i64,
    );
    (antennas, grid_shape)
}

fn get_antinodes2((x1, y1): Coord, (x2, y2): Coord) -> Vec<Coord> {
    // TODO: check for internal antinodes
    vec![(2 * x1 - x2, 2 * y1 - y2), (2 * x2 - x1, 2 * y2 - y1)]
}

fn get_antinodes(ps: &Vec<Coord>) -> HashSet<Coord> {
    ps.iter()
        .enumerate()
        .flat_map(|(i, a)| ps.iter().skip(i + 1).map(move |b| (a, b))) // combinations(ps, ps)
        .flat_map(|(p1, p2)| get_antinodes2(*p1, *p2))
        .collect()
}
