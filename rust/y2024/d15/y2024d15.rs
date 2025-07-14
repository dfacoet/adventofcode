use std::collections::HashMap;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (mut pos, mut grid, moves) = parse_input(input)?;

    for m in moves {
        let next_pos = m.next(pos);
        let mut c = next_pos;
        let mut box_to = None;
        while matches!(grid[c.0][c.1], Cell::BoxLeft) {
            c = m.next(c);
            box_to = Some(c);
        }
        match grid[c.0][c.1] {
            Cell::Empty => {
                grid[pos.0][pos.1] = Cell::Empty;
                grid[next_pos.0][next_pos.1] = Cell::Robot;
                pos = next_pos;
                if let Some(box_to) = box_to {
                    grid[box_to.0][box_to.1] = Cell::BoxLeft;
                }
            }
            Cell::Wall => (), // do nothing
            _ => return Err("something is wrong".into()),
        }
    }

    Ok(total_gps(&grid).to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (mut pos, grid, moves) = parse_input(input)?;
    let mut grid = expand_grid(&grid);
    pos.1 *= 2;

    for m in moves {
        let mut moving = true;
        let mut move_map = HashMap::new();
        let mut queue = vec![pos];

        while let Some(c) = queue.pop() {
            let (i, j) = m.next(c);
            move_map.insert(c, ((i, j), grid[c.0][c.1]));
            match grid[i][j] {
                Cell::Robot => panic!("Trying to move into a robot aat {:?}", (i, j)),
                Cell::Empty => continue,
                Cell::Wall => {
                    moving = false;
                    break; // would like to use in while condition, but it's unstable
                }
                Cell::BoxLeft => {
                    let right = (i, j + 1);
                    if !move_map.contains_key(&right) {
                        queue.push(right);
                    }
                }
                Cell::BoxRight => {
                    let left = (i, j - 1);
                    if !move_map.contains_key(&left) {
                        queue.push(left);
                    }
                }
            }
            queue.push((i, j));
        }

        if moving {
            pos = m.next(pos);
            move_map.keys().for_each(|&(i, j)| grid[i][j] = Cell::Empty);
            move_map.values().for_each(|&((i, j), c)| {
                grid[i][j] = c;
            });
        }
    }

    Ok(total_gps(&grid).to_string())
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Robot,
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '@' => Cell::Robot,
            '#' => Cell::Wall,
            'O' => Cell::BoxLeft,
            '.' => Cell::Empty,
            _ => panic!("Invalid character in grid"),
        }
    }
}

#[derive(Debug)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Move::Up,
            '>' => Move::Right,
            'v' => Move::Down,
            '<' => Move::Left,
            _ => panic!("Invalid character in moves: {}", c),
        }
    }

    fn next(&self, (i, j): Coord) -> Coord {
        match self {
            Move::Up => (i - 1, j),
            Move::Right => (i, j + 1),
            Move::Down => (i + 1, j),
            Move::Left => (i, j - 1),
        }
    }
}

type Coord = (usize, usize);
type Input = (Coord, Vec<Vec<Cell>>, Vec<Move>);

fn parse_input(input: String) -> Result<Input, Box<dyn std::error::Error>> {
    let v = input.split("\n\n").collect::<Vec<_>>();
    let (grid, moves) = if v.len() == 2 {
        (v[0], v[1])
    } else {
        return Err("Invalid input".into());
    };
    let grid: Vec<Vec<Cell>> = grid
        .lines()
        .map(|l| l.chars().map(Cell::from_char).collect())
        .collect();
    let moves = moves
        .lines()
        .flat_map(|l| l.chars()) // get rid of newlines
        .map(Move::from_char)
        .collect();
    let pos = grid
        .iter()
        .enumerate()
        .filter_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(c, cell)| match cell {
                    Cell::Robot => Some((r, c)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();
    Ok((pos, grid, moves))
}

fn total_gps(grid: &[Vec<Cell>]) -> usize {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, cell)| match cell {
                Cell::BoxLeft => 100 * i + j,
                _ => 0,
            })
        })
        .sum()
}

fn expand_grid(grid: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    grid.iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| match cell {
                    Cell::Robot => [Cell::Robot, Cell::Empty],
                    Cell::Empty => [Cell::Empty, Cell::Empty],
                    Cell::Wall => [Cell::Wall, Cell::Wall],
                    Cell::BoxLeft => [Cell::BoxLeft, Cell::BoxRight],
                    Cell::BoxRight => panic!("Grid is already expanded"),
                })
                .collect()
        })
        .collect()
}
