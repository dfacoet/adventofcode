use std::collections::HashSet;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let grid = parse_input(input)?;
    let zeros = grid.iter().enumerate().flat_map(|(i, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(j, h)| if *h == 0 { Some((i, j)) } else { None })
    });
    Ok(zeros
        .map(|p| find_trails(&p, &grid))
        .sum::<usize>()
        .to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

type Grid = Vec<Vec<u32>>;

fn parse_input(input: String) -> Result<Grid, Box<dyn std::error::Error>> {
    let grid: Grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    // pad with MAXs
    let pad = u32::MAX;
    let width = grid.first().ok_or("empty grid")?.len() + 2;
    let grid = std::iter::once(vec![pad; width])
        .chain(grid.into_iter().map(|line| {
            std::iter::once(pad)
                .chain(line)
                .chain(std::iter::once(pad))
                .collect()
        }))
        .chain(std::iter::once(vec![pad; width]))
        .collect();

    Ok(grid)
}

fn find_trails(start: &(usize, usize), grid: &Grid) -> usize {
    let mut live_trails = Vec::new();
    // assume that start is 0 without checking
    live_trails.push(vec![(*start, 0)]);

    let mut ends = HashSet::new();
    while let Some(trail) = live_trails.pop() {
        let ((x, y), h) = trail.last().unwrap();
        for (nx, ny) in [(x - 1, *y), (*x, y + 1), (x + 1, *y), (*x, y - 1)] {
            let nh = grid.get(nx).unwrap().get(ny).unwrap();
            if *nh == h + 1 {
                if *nh == 9 {
                    ends.insert((nx, ny));
                } else {
                    let mut new_trail = trail.clone();
                    let pos = (nx, ny);
                    new_trail.push((pos, *nh));
                    live_trails.push(new_trail);
                }
            }
        }
    }
    ends.len()
}
