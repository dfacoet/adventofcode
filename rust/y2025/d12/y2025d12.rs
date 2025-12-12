use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (areas, spaces) = parse_input(input)?;
    Ok(spaces.iter().filter(|s| s.fits(&areas)).count().to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok("Merry Christmas".into())
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

impl Space {
    fn area(&self) -> usize {
        self.shape.0 * self.shape.1
    }

    fn fits(&self, shapes: &[usize; 6]) -> bool {
        let required_area: usize = shapes
            .iter()
            .zip(self.counts.iter())
            .map(|(s, n)| n * s)
            .sum();
        required_area <= self.area()
    }
}

fn parse_input(input: String) -> Result<([usize; 6], Vec<Space>), Box<dyn std::error::Error>> {
    let lines = input.trim().lines().collect::<Vec<_>>();
    let blocks = lines.split(|s| s.is_empty()).collect::<Vec<_>>();
    if blocks.len() != 7 {
        return Err("Invalid input".into());
    }
    let shape_areas = blocks
        .iter()
        .take(6)
        .map(|b| {
            b.iter()
                .map(|r| r.chars().filter(|c| *c == '#').count())
                .sum()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let spaces = blocks
        .last()
        .ok_or("Invalid input")?
        .iter()
        .map(|s| Space::from_str(s))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((shape_areas, spaces))
}
