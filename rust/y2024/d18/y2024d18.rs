use std::collections::{HashSet, VecDeque};

type Coord = (i8, i8);
const SIZE: Coord = (71, 71);
const TARGET: Coord = (SIZE.0 - 1, SIZE.1 - 1);

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = parse_input(input)?
        .into_iter()
        .take(1024)
        .collect::<HashSet<_>>();
    let mut visited: HashSet<Coord> = HashSet::from_iter([(0, 0)]);
    let mut paths: VecDeque<(usize, Coord)> = VecDeque::from_iter([(0, (0, 0))]); // [(len, head)]
    while let Some((n_steps, (i, j))) = paths.pop_front() {
        let n_steps = n_steps + 1;
        for next in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if next == TARGET {
                return Ok(n_steps.to_string());
            } else if in_range(&next) && !bytes.contains(&next) && !visited.contains(&next) {
                visited.insert(next);
                paths.push_back((n_steps, next));
            }
        }
    }
    Err("No path found".into())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
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
                Ok((v[1], v[0]))
            } else {
                Err("Invalid input".into())
            }
        })
        .collect()
}

fn in_range((i, j): &Coord) -> bool {
    0 <= *i && *i < SIZE.0 && 0 <= *j && *j < SIZE.1
}
