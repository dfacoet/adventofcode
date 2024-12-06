use core::panic;
use std::collections::HashSet;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 1
    let mut obstacles = HashSet::new();
    let mut guard_pos = (0, 0);
    let mut guard_dir = 0;
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

    let mut visited = HashSet::new();
    visited.insert(guard_pos);
    while let Some((new_pos, new_dir)) = get_next(guard_pos, guard_dir, &obstacles, grid_bounds) {
        (guard_pos, guard_dir) = (new_pos, new_dir);
        visited.insert(guard_pos);
    }

    Ok(format!("{}", visited.len()))
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

type Coord = (usize, usize);

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
