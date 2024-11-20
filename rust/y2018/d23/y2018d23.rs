use regex::Regex;
use std::{cmp::max, cmp::min, str::FromStr};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let bots = parse_input(input)?;
    let strongest = bots.iter().max_by_key(|bot| bot.r).ok_or("No bots found")?;

    let n_in_range = bots.iter().filter(|bot| strongest.in_range(bot)).count();

    Ok(n_in_range.to_string())
}

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
    let mut region: Region = ([i32::MIN / 10; 3], [i32::MAX / 10; 3]);

    // Recursively select the half region intersecting the most bots
    while dist(&region.0, &region.1) > 3 {
        println!("{:?}", region);
        println!("{:?}", dist(&region.0, &[0, 0, 0]));
        println!("{:?}", dist(&region.1, &[0, 0, 0]));
        let new_regions = split(&region);
        region = *new_regions
            .iter()
            .max_by_key(|r| bots.iter().filter(|bot| bot.in_range_region(r)).count())
            .ok_or("No regions")?;
    }

    Ok(format!("{:?}, {}", region, volume(region)))
}

// 597345651 is too high
// 644245088
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
        .map(|(a, b)| (a - b).abs() as u64)
        .sum()
}

impl Bot {
    fn in_range(&self, other: &Bot) -> bool {
        dist(&self.pos, &other.pos) <= self.r
    }

    fn in_range_region(&self, r: &Region) -> bool {
        true
    }
}

fn parse_input(input: String) -> Result<Vec<Bot>, Box<dyn std::error::Error>> {
    input.lines().map(Bot::from_str).collect()
}

// Represent a rectangular region of space as two opposite vertices
type Region = (Coord, Coord);

fn volume(r: Region) -> u64 {
    r.1.iter()
        .zip(r.0.iter())
        .map(|(a, b)| (a - b).abs() as u64)
        .product()
}

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
