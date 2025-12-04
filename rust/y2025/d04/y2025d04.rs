use std::collections::HashSet;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let positions = get_rolls_set(input);
    let result = positions
        .iter()
        .filter(|p| is_accessible(p, &positions))
        .count();
    Ok(result.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut positions = get_rolls_set(input);
    let n_removed = std::iter::from_fn(|| {
        let n = remove_rolls(&mut positions);
        if n == 0 {
            None
        } else {
            Some(n)
        }
    })
    .sum::<usize>();
    Ok(n_removed.to_string())
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

fn is_accessible(p: &(usize, usize), positions: &HashSet<(usize, usize)>) -> bool {
    // neighborg includes p itself, so compare with 5 (assumes p is a roll)
    neighbors(*p).filter(|n| positions.contains(n)).count() < 5
}

fn remove_rolls(positions: &mut HashSet<(usize, usize)>) -> usize {
    let to_remove: Vec<_> = positions
        .iter()
        .filter(|p| is_accessible(p, positions))
        .copied()
        .collect();
    to_remove.iter().for_each(|p| {
        positions.remove(p);
    });
    to_remove.len()
}
