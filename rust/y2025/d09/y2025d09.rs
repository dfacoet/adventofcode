use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let tiles = parse_input(input)?;
    let max_area = tiles
        .iter()
        .flat_map(|a| tiles.iter().map(|b| Rectangle::from_corners(a, b).area()))
        .max()
        .ok_or("Input is empty")?;
    Ok(max_area.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let tiles = parse_input(input)?;
    let floor = Floor::from_red_tiles(&tiles)?;

    let max_area = tiles
        .iter()
        .flat_map(|a| {
            tiles.iter().filter_map(|b| {
                let r = Rectangle::from_corners(a, b);
                if floor.contains_rectangle(&r).unwrap() {
                    Some(r.area())
                } else {
                    None
                }
            })
        })
        .max()
        .ok_or("Input is empty")?;

    Ok(max_area.to_string())
}

type Coords = (u64, u64);

fn parse_input(input: String) -> Result<Vec<Coords>, Box<dyn std::error::Error>> {
    input
        .trim()
        .lines()
        .map(|line| -> Result<_, _> {
            let (a, b) = line.split_once(',').ok_or("Invalid input")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect()
}

struct ContractMap {
    xmap: HashMap<u64, usize>,
    ymap: HashMap<u64, usize>,
}

impl ContractMap {
    fn mapx(&self, x: &u64) -> Result<usize, Box<dyn std::error::Error>> {
        self.xmap.get(x).ok_or("Not found".into()).copied()
    }
    fn mapy(&self, y: &u64) -> Result<usize, Box<dyn std::error::Error>> {
        self.ymap.get(y).ok_or("Not found".into()).copied()
    }
    fn map(&self, (x, y): &Coords) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        Ok((self.mapx(x)?, self.mapy(y)?))
    }
    fn map_rectangle(&self, r: &Rectangle) -> Result<CRectangle, Box<dyn std::error::Error>> {
        Ok(CRectangle {
            bottom: self.mapy(&r.bottom)?,
            top: self.mapy(&r.top)?,
            left: self.mapx(&r.left)?,
            right: self.mapx(&r.right)?,
        })
    }

    fn from_tiles(tiles: &[Coords]) -> Self {
        ContractMap {
            xmap: contract_map(tiles.iter().map(|(x, _)| *x)),
            ymap: contract_map(tiles.iter().map(|(_, y)| *y)),
        }
    }
}

struct Rectangle {
    bottom: u64,
    top: u64,
    left: u64,
    right: u64,
}
struct CRectangle {
    bottom: usize,
    top: usize,
    left: usize,
    right: usize,
}

impl Rectangle {
    fn from_corners((x1, y1): &Coords, (x2, y2): &Coords) -> Self {
        Rectangle {
            left: *x1.min(x2),
            right: *x1.max(x2),
            bottom: *y1.min(y2),
            top: *y1.max(y2),
        }
    }
    fn area(&self) -> u64 {
        (self.top - self.bottom + 1) * (self.right - self.left + 1)
    }
}

struct Floor {
    contract_map: ContractMap,
    inside: HashSet<(usize, usize)>,
}

impl Floor {
    fn from_red_tiles(tiles: &[Coords]) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_map = ContractMap::from_tiles(tiles);
        let mapped_tiles = tiles
            .iter()
            .map(|p| contract_map.map(p))
            .collect::<Result<Vec<_>, _>>()?;
        // start with the perimeter
        let mut inside: HashSet<_> = windows(&mapped_tiles)
            .flat_map(|((x1, y1), (x2, y2))| {
                range(x1, x2).flat_map(|x| range(y1, y2).map(move |y| (x, y)))
            })
            .collect();
        // Get the first tile on the left boundary
        let (i, v) = mapped_tiles
            .iter()
            .enumerate()
            .find(|(_, (x, _))| *x == 1)
            .ok_or("No tiles")?;
        let next = mapped_tiles[i + 1];
        // TODO: If assertion fails, the two tiles we are looking for
        // are the first and last one
        assert!(next.0 == v.0);
        // Get the first inner tile, depending on travel direction
        let s = if next.1 > v.1 {
            // CW -> upper right
            (v.0 + 1, v.1 + 1)
        } else {
            // CCW -> lower right
            (v.0 + 1, v.1 - 1)
        };
        // flood the inside
        let mut q = vec![s];
        inside.insert(s);
        while let Some((x, y)) = q.pop() {
            for neighbor in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if inside.insert(neighbor) {
                    q.push(neighbor);
                }
            }
        }

        Ok(Floor {
            contract_map,
            inside,
        })
    }

    fn contains_rectangle(&self, r: &Rectangle) -> Result<bool, Box<dyn std::error::Error>> {
        let r = self.contract_map.map_rectangle(r)?;
        let mut boundary = (r.bottom..=r.top)
            .flat_map(|y| [(r.left, y), (r.right, y)])
            .chain((r.left..=r.right).flat_map(|x| [(x, r.bottom), (x, r.top)]));
        Ok(!boundary.any(|p| !self.inside.contains(&p)))
    }
}

fn contract_map(xs: impl Iterator<Item = u64>) -> HashMap<u64, usize> {
    let set: HashSet<_> = xs.flat_map(|x| [x - 1, x, x + 1]).collect();
    let mut xs: Vec<_> = set.into_iter().collect();
    xs.sort();
    xs.into_iter().enumerate().map(|(i, x)| (x, i)).collect()
}

fn windows<T>(xs: &[T]) -> impl Iterator<Item = (&T, &T)> {
    xs.iter().skip(1).chain(xs.iter().take(1)).zip(xs.iter())
}

fn range<T: Ord + Copy>(a: &T, b: &T) -> std::ops::RangeInclusive<T> {
    *a.min(b)..=*a.max(b)
}
