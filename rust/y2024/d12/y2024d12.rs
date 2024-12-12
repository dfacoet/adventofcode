use std::collections::HashSet;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let grid = parse_input(input)?;

    let mut total_price = 0;
    let mut assigned = HashSet::new();
    for i in 1..grid.len() - 1 {
        // skip first and last (padding)
        for j in 1..grid[i].len() - 1 {
            if assigned.contains(&(i, j)) {
                continue;
            }
            let mut q = Vec::new();
            q.push((i, j));
            let mut region = HashSet::new();
            let mut perimeter = 0;
            let plant_type = grid[i][j];
            while let Some((i, j)) = q.pop() {
                if region.contains(&(i, j)) {
                    continue;
                }
                region.insert((i, j));
                for nplot in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
                    if region.contains(&nplot) {
                        continue;
                    }
                    if grid[nplot.0][nplot.1] == plant_type {
                        q.push(nplot);
                    } else {
                        perimeter += 1;
                    }
                }
            }
            total_price += perimeter * region.len();
            assigned.extend(region);
        }
    }
    Ok(total_price.to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

type Grid = Vec<Vec<char>>;

fn parse_input(input: String) -> Result<Grid, Box<dyn std::error::Error>> {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    // pad with .
    // TODO: write generic padding function
    let pad = '.';
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
