use std::collections::HashSet;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (guard_pos, obstacles, grid_bounds) = parse_input(input)?;

    run_guard(guard_pos, &obstacles, grid_bounds)
        .map(|visited| visited.len().to_string())
        .ok_or_else(|| "Guard is stuck in a loop".into())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (guard_pos, obstacles, grid_bounds) = parse_input(input)?;
    let visited = run_guard(guard_pos, &obstacles, grid_bounds).unwrap();
    let mut count = 0;
    for (i, j) in visited {
        let mut new_obstacles = obstacles.clone();
        if (i, j) != guard_pos && new_obstacles.insert((i, j)) {
            match run_guard(guard_pos, &new_obstacles, grid_bounds) {
                Some(_) => { // guard is not stuck in a loop
                }
                None => count += 1,
            }
        }
    }
    Ok(count.to_string())
}

type Coord = (usize, usize);

fn parse_input(
    input: String,
) -> Result<(Coord, HashSet<Coord>, Coord), Box<dyn std::error::Error>> {
    let mut obstacles = HashSet::new();
    let mut guard_pos = (0, 0);
    let mut grid_bounds = (0, 0);

    for (i, row) in input.split('\n').enumerate() {
        for (j, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((i + 1, j + 1));
                }
                '^' => guard_pos = (i + 1, j + 1),
                '.' => {}
                _ => return Err("Invalid character".into()),
            }
        }
        if !row.is_empty() {
            grid_bounds = (i + 2, row.len() + 1);
        }
    }

    Ok((guard_pos, obstacles, grid_bounds))
}

fn get_next(pos: Coord, dir: u8, obstacles: &HashSet<Coord>, bounds: Coord) -> Option<(Coord, u8)> {
    let next_cell = match dir {
        0 => (pos.0 - 1, pos.1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 + 1, pos.1),
        3 => (pos.0, pos.1 - 1),
        _ => panic!("Invalid direction"),
    };
    if next_cell.0 == 0 || next_cell.1 == 0 || next_cell.0 == bounds.0 || next_cell.1 == bounds.1 {
        None
    } else if obstacles.contains(&next_cell) {
        Some((pos, (dir + 1) % 4))
    } else {
        Some((next_cell, dir))
    }
}

fn run_guard(
    initial_pos: Coord,
    obstacles: &HashSet<Coord>,
    grid_bounds: Coord,
) -> Option<HashSet<Coord>> {
    let mut guard_dir = 0;
    let mut guard_pos = initial_pos;

    let mut visited = HashSet::new();
    visited.insert((guard_pos, guard_dir));
    while let Some((new_pos, new_dir)) = get_next(guard_pos, guard_dir, obstacles, grid_bounds) {
        (guard_pos, guard_dir) = (new_pos, new_dir);
        if !visited.insert((guard_pos, guard_dir)) {
            return None;
        }
    }
    Some(visited.iter().map(|(p, _)| *p).collect())
}
