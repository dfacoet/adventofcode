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
                if !region.insert((i, j)) {
                    continue;
                }
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

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let grid = parse_input(input)?;

    let mut total_price = 0;
    let mut assigned = HashSet::new();
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if assigned.contains(&(i, j)) {
                continue;
            }
            let mut q = Vec::new();
            q.push((i, j));
            let mut region = HashSet::new();
            let plant_type = grid[i][j];
            let mut n_sides = 0;
            while let Some((i, j)) = q.pop() {
                if region.contains(&(i, j)) {
                    continue;
                }
                region.insert((i, j));
                for nplot in ncoords(&(i, j)) {
                    if grid[nplot.0][nplot.1] == plant_type && !region.contains(&nplot) {
                        q.push(nplot);
                    }
                }
                n_sides += count_corners(&(i, j), &grid)
            }

            println!("Region {plant_type}: {} * {n_sides}", region.len());
            total_price += n_sides * region.len();
            assigned.extend(region);
        }
    }
    Ok(total_price.to_string())
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

fn ncoords((i, j): &(usize, usize)) -> Vec<(usize, usize)> {
    vec![(i - 1, *j), (*i, j + 1), (i + 1, *j), (*i, j - 1)]
}

fn count_corners(p: &(usize, usize), grid: &Grid) -> usize {
    let plant_type = grid[p.0][p.1];
    let ns = ncoords(p);
    let nds = vec![
        (p.0 - 1, p.1 + 1),
        (p.0 + 1, p.1 + 1),
        (p.0 + 1, p.1 - 1),
        (p.0 - 1, p.1 - 1),
    ];

    // Iterate over four corners with two consecutive neighbours (n1, n2) of cc and
    // the diagonal neighbour nd.
    // Count the corner if
    // 1. n1!=cc, n2!=cc, regardless of nd.
    // 2. n1==n2==cc && nd != cc.
    // Otherwise there is no corner (3,4) or there is a corner, but it's counted
    // by a neighbour (n1 in 5)
    // Examples:  nd.n2
    //            n1.cc
    // 1.   2.   3.  4.  5.
    // ? X  X|O  OO  XX  O|X
    //   =  =        ==    =
    // X|O  O O  OO  OO  O O

    ns.iter()
        .zip(ns.iter().cycle().skip(1))
        .zip(nds)
        .filter(|((n1, n2), nd)| {
            (grid[n1.0][n1.1] != plant_type && grid[n2.0][n2.1] != plant_type)
                || (grid[n1.0][n1.1] == plant_type
                    && grid[n2.0][n2.1] == plant_type
                    && grid[nd.0][nd.1] != plant_type)
        })
        .count()
}
