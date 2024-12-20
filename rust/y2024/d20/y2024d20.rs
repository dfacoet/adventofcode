use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (start, end, tracks) = parse_input(input)?;
    let path = find_path(&start, &end, &tracks)?; // pos -> time to target

    Ok(find_cheats(&path, 2)
        .filter(|x| *x >= 100)
        .count()
        .to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (start, end, tracks) = parse_input(input)?;
    let path = find_path(&start, &end, &tracks)?;

    Ok(find_cheats(&path, 20)
        .filter(|x| *x >= 100)
        .count()
        .to_string())
}

type Coord = (usize, usize);

fn parse_input(
    input: String,
) -> Result<(Coord, Coord, HashSet<Coord>), Box<dyn std::error::Error>> {
    let start = find_char(&input, 'S')?;
    let end = find_char(&input, 'E')?;
    let mut tracks: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter_map(move |(j, c)| if c == '.' { Some((i, j)) } else { None })
        })
        .collect();
    tracks.insert(start);
    tracks.insert(end);
    Ok((start, end, tracks))
}

fn find_char(input: &str, needle: char) -> Result<Coord, Box<dyn std::error::Error>> {
    input
        .lines()
        .enumerate()
        .find_map(|(i, line)| line.chars().position(|c| c == needle).map(|j| (i, j)))
        .ok_or(format!("{needle} not found").into())
}

fn neighbors_d((i, j): &Coord, max_d: usize) -> impl Iterator<Item = (Coord, usize)> + '_ {
    (2..=max_d).flat_map(move |d| {
        (0..=d).flat_map(move |di| {
            let dj = d - di;
            let ip = i + di;
            let im = if di > 0 { i.checked_sub(di) } else { None };
            let jp = j + dj;
            let jm = if dj > 0 { j.checked_sub(dj) } else { None };
            [
                Some((ip, jp)),
                im.map(|ni| (ni, jp)),
                jm.map(|nj| (ip, nj)),
                match (im, jm) {
                    (Some(ni), Some(nj)) => Some((ni, nj)),
                    _ => None, // nicer impl. of monadic ops?
                },
            ]
            .into_iter()
            .flatten()
            .map(move |n| (n, d))
        })
    })
}

fn find_path(
    start: &Coord,
    end: &Coord,
    tracks: &HashSet<Coord>,
) -> Result<HashMap<Coord, usize>, Box<dyn std::error::Error>> {
    let mut pos = *start;
    let mut path = vec![pos];
    while pos != *end {
        let neighbors = [
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
        ];
        let ns: Vec<_> = neighbors
            .iter()
            .filter(|n| tracks.contains(n) && (path.len() < 2 || path[path.len() - 2] != **n))
            .collect();
        pos = match ns.len() {
            0 => return Err("No path found".into()),
            1 => *ns[0],
            _ => return Err("Path is not unique".into()),
        };
        path.push(pos);
    }

    Ok(path
        .iter()
        .rev()
        .enumerate()
        .map(|(t, &p)| (p, t))
        .collect())
}

fn find_cheats(path: &HashMap<Coord, usize>, max_dist: usize) -> impl Iterator<Item = usize> + '_ {
    path.iter()
        .flat_map(move |(p, t1)| {
            neighbors_d(p, max_dist)
                .filter_map(move |(n, d)| path.get(&n).map(move |t2| t1.checked_sub(t2 + d)))
        })
        .flatten() // unpack Somes, filter out Nones
}
