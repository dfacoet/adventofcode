use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (shapes, spaces) = parse_input(input)?;
    println!("{} {}", shapes.len(), spaces.len());
    println!("{:?}", shapes[0]);
    println!("{:?}", spaces[0]);
    Err("Solution not implemented".into())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

#[derive(Debug)]
struct Shape {
    grid: [[bool; 3]; 3],
}

impl Shape {
    fn from_strs(block: &&[&str]) -> Self {
        let mut grid = [[false; 3]; 3];
        for (row_idx, line) in block.iter().skip(1).enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                if row_idx < 3 && col_idx < 3 {
                    grid[row_idx][col_idx] = ch == '#';
                }
            }
        }

        Shape { grid }
    }
}

#[derive(Debug)]
struct Space {
    shape: (usize, usize),
    counts: [usize; 6],
}

impl FromStr for Space {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape_str, counts_str) = s.split_once(": ").ok_or("Invalid line")?;
        let (w, h) = shape_str.split_once('x').ok_or("Invalid line")?;
        let count_parts: Vec<&str> = counts_str.split_whitespace().collect();
        if count_parts.len() != 6 {
            return Err("Expected exactly 6 counts".into());
        }
        let mut counts = [0; 6];
        for (i, part) in count_parts.iter().enumerate() {
            counts[i] = part.parse()?;
        }

        Ok(Space {
            shape: (w.parse()?, h.parse()?),
            counts,
        })
    }
}

fn parse_input(input: String) -> Result<(Vec<Shape>, Vec<Space>), Box<dyn std::error::Error>> {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let blocks = lines.split(|s| *s == "").collect::<Vec<_>>();
    let shapes = blocks
        .iter()
        .take_while(|b| {
            b[0].chars().nth(0).map_or(false, |c| c.is_ascii_digit())
                && b[0].chars().nth(1) == Some(':')
        })
        .map(Shape::from_strs)
        .collect();

    let spaces = blocks
        .last()
        .ok_or("Invalid input")?
        .into_iter()
        .map(|s| Space::from_str(*s))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((shapes, spaces))
}
