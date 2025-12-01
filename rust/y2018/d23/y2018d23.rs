use regex::Regex;
use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bots = parse_input(input)?;
    let strongest = bots.iter().max_by_key(|bot| bot.r).ok_or("No bots found")?;

    let n_in_range = bots.iter().filter(|bot| strongest.in_range(bot)).count();

    Ok(n_in_range.to_string())
}

/// Bisection method: take the whole space, and iteratively split it in two,
/// and keep the region intersecting the most Bot Manhattan balls.
/// I don't think this is guaranteed to work an all inputs (local maximum?),
/// and it needs some cleaning up (especially around inclusive/exclusive ranges etc),
/// but it works on my input.
pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bots = parse_input(input)?;

    // // Set region to the rectangular envelope of the bot positions
    // let mut region: Region = ([i32::MAX; 3], [i32::MIN; 3]);
    // for bot in bots.iter() {
    //     for i in 0..3 {
    //         region.0[i] = min(region.0[i], bot.pos[i]);
    //         region.1[i] = max(region.1[i], bot.pos[i]);
    //     }
    // }
    let mut region: Region = ([i32::MIN / 2; 3], [i32::MAX / 2; 3]);

    // Recursively select the half region intersecting the most bots
    while dist(&region.0, &region.1) > 3 {
        let new_regions = split(&region);
        region = *new_regions
            .iter()
            .max_by_key(|r| bots.iter().filter(|bot| bot.in_range_region(r)).count())
            .ok_or("No regions")?;
    }

    // When the region is small enough, iterate over the points in the region.
    // This is a lazy way to avoid bisecting properly

    let points = region_points(&region);
    let max_point = points
        .iter()
        .max_by_key(|p| {
            (
                bots.iter().filter(|bot| dist(p, &bot.pos) <= bot.r).count(),
                dist(p, &[0, 0, 0]),
            )
        })
        .ok_or("No points in region")?;

    Ok(dist(max_point, &[0, 0, 0]).to_string())
}

type Coord = [i32; 3];

struct Bot {
    pos: Coord,
    r: u64,
}

impl FromStr for Bot {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)")?;
        let caps = re.captures(s).ok_or("Invalid input format")?;

        let pos = [caps[1].parse()?, caps[2].parse()?, caps[3].parse()?];
        let r = caps[4].parse()?;

        Ok(Bot { pos, r })
    }
}

fn dist(p1: &Coord, p2: &Coord) -> u64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| (a - b).unsigned_abs() as u64)
        .sum()
}

impl Bot {
    fn in_range(&self, other: &Bot) -> bool {
        dist(&self.pos, &other.pos) <= self.r
    }

    fn in_range_region(&self, r: &Region) -> bool {
        // The region has points in range if either
        // - one of the vertices of the bot's Manhattan ball is in the region
        // - one of the corners of the region is in the Manhattan ball
        for i in 0..3 {
            let mut vertex = self.pos;
            vertex[i] += self.r as i32;
            if region_contains(r, &vertex) {
                return true;
            };
            let mut vertex = self.pos;
            vertex[i] -= self.r as i32;
            if region_contains(r, &vertex) {
                return true;
            };
        }
        for i in 0..8 {
            let mut corner = [0; 3];
            #[allow(clippy::needless_range_loop)]
            for j in 0..3 {
                // i ~3 bits; the j-th bit of i determines
                // whether the j-th coordinate is taken from r.0 or r.1
                // This generates all 8 vertices iterating over i
                corner[j] = if (i & (1 << j)) == 0 { r.0[j] } else { r.1[j] };
            }
            if dist(&self.pos, &corner) <= self.r {
                return true;
            };
        }
        false
    }
}

fn parse_input(input: String) -> Result<Vec<Bot>, Box<dyn std::error::Error>> {
    input.lines().map(Bot::from_str).collect()
}

// Represent a rectangular region of space as two opposite vertices
type Region = (Coord, Coord);

fn split(r: &Region) -> Vec<Region> {
    // split the region in two along the longest dimension
    let (max_d, l) =
        r.1.iter()
            .zip(r.0.iter())
            .map(|(a, b)| a - b)
            .enumerate()
            .max_by_key(|(_, l)| *l)
            .unwrap();
    let c = r.0[max_d] + l / 2;
    let mut v1 = r.1;
    let mut v2 = r.0;
    v1[max_d] = c;
    v2[max_d] = c;

    vec![(r.0, v1), (v2, r.1)]
}

fn region_contains(r: &Region, p: &Coord) -> bool {
    p.iter()
        .zip(r.0.iter().zip(r.1.iter()))
        .all(|(x, (c1, c2))| c1 <= x && x <= c2)
}

fn region_points(r: &Region) -> Vec<Coord> {
    let mut points = vec![];
    for i in r.0[0]..=r.1[0] {
        for j in r.0[1]..=r.1[1] {
            for k in r.0[2]..=r.1[2] {
                points.push([i, j, k]);
            }
        }
    }
    points
}
