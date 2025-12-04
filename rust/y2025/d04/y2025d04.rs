use std::collections::HashSet;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let positions = get_rolls_set(input);
    let result = positions
        .iter()
        .filter(|p| count_neighbours(p, &positions) < 4)
        .count();
    Ok(result.to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

fn get_rolls_set(input: String) -> HashSet<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.char_indices().filter_map(
                move |(j, c)| {
                    if c == '@' {
                        Some((i + 1, j + 1))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

fn neighbors((i, j): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    (i - 1..=i + 1).flat_map(move |x| (j - 1..=j + 1).map(move |y| (x, y)))
}

fn count_neighbours(p: &(usize, usize), positions: &HashSet<(usize, usize)>) -> usize {
    neighbors(*p).filter(|n| positions.contains(n)).count() - 1
}
