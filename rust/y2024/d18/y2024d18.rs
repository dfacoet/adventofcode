use std::collections::{HashSet, VecDeque};

type Coord = (i8, i8);
const SIZE: Coord = (71, 71);
const TARGET: Coord = (SIZE.0 - 1, SIZE.1 - 1);

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bytes: HashSet<_> = parse_input(input)?.into_iter().take(1024).collect();
    shortest_path(bytes)
        .map(|n| n.to_string())
        .ok_or("No path found".into())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let all_bytes = parse_input(input)?;
    // Improvement: bisect on i
    for (i, last_byte) in all_bytes.iter().enumerate() {
        let bytes: HashSet<_> = all_bytes.clone().into_iter().take(i + 1).collect();
        if shortest_path(bytes).is_none() {
            return Ok(format!("{},{}", last_byte.0, last_byte.1));
        }
    }
    Err("Path not blocked".into())
}

fn parse_input(input: String) -> Result<Vec<Coord>, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(|s| {
            let v = s
                .split(',')
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()?;
            if v.len() == 2 {
                Ok((v[0], v[1]))
            } else {
                Err("Invalid input".into())
            }
        })
        .collect()
}

fn in_range((i, j): &Coord) -> bool {
    0 <= *i && *i < SIZE.0 && 0 <= *j && *j < SIZE.1
}

fn shortest_path(bytes: HashSet<Coord>) -> Option<usize> {
    let mut visited: HashSet<Coord> = HashSet::from_iter([(0, 0)]);
    let mut paths: VecDeque<(usize, Coord)> = VecDeque::from_iter([(0, (0, 0))]);
    // [(len, head)]
    while let Some((n_steps, (i, j))) = paths.pop_front() {
        let n_steps = n_steps + 1;
        for next in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if next == TARGET {
                return Some(n_steps);
            } else if in_range(&next) && !bytes.contains(&next) && !visited.contains(&next) {
                visited.insert(next);
                paths.push_back((n_steps, next));
            }
        }
    }
    None
}
