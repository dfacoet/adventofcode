use std::collections::{HashMap, HashSet};

type Coord = (i64, i64);

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (antennas, grid_shape) = parse_input(input);

    let antinodes: HashSet<Coord> = antennas
        .values()
        .flat_map(get_antinodes)
        .filter(|p| in_grid(*p, grid_shape))
        .collect();

    Ok(antinodes.len().to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (antennas, grid_shape) = parse_input(input);

    let antinodes: HashSet<Coord> = antennas
        .values()
        .flat_map(|ps| get_antinodes2(ps, grid_shape))
        .collect();

    Ok(antinodes.len().to_string())
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

// TODO: make generic and move to shared library
// Consider adding trait for iterators, or using external crate
fn pairs(xs: &Vec<Coord>) -> impl Iterator<Item = (&Coord, &Coord)> {
    xs.iter()
        .enumerate()
        .flat_map(|(i, a)| xs.iter().skip(i + 1).map(move |b| (a, b)))
}

fn get_antinodes(ps: &Vec<Coord>) -> HashSet<Coord> {
    pairs(ps)
        .flat_map(|((x1, y1), (x2, y2))| {
            // TODO: check for internal antinodes
            vec![(2 * x1 - x2, 2 * y1 - y2), (2 * x2 - x1, 2 * y2 - y1)]
        })
        .collect()
}

fn in_grid((x, y): Coord, (xmax, ymax): Coord) -> bool {
    x >= 0 && x < xmax && y >= 0 && y < ymax
}

fn get_antinodes2(ps: &Vec<Coord>, grid_shape: Coord) -> HashSet<Coord> {
    pairs(ps)
        .flat_map(|((x1, y1), (x2, y2))| {
            let (dx, dy) = (x2 - x1, y2 - y1);
            (0..)
                .map(move |k| (x1 - k * dx, y1 - k * dy))
                .take_while(|p| in_grid(*p, grid_shape))
                .chain(
                    (0..)
                        .map(move |k| (x1 + k * dx, y1 + k * dy))
                        .take_while(|p| in_grid(*p, grid_shape)),
                )
        })
        .collect()
}
