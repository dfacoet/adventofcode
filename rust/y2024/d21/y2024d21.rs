use std::collections::{HashMap, VecDeque};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    solve(input, 3)
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    solve(input, 26)
}

fn solve(input: String, depth: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut cache = HashMap::new();
    Ok(input
        .lines()
        .map(|code| {
            match (
                code[..3].parse::<usize>(),
                shortest_sequence(code, depth, depth, &mut cache),
            ) {
                (Ok(code_n), Ok(shortest_seq)) => Ok(code_n * shortest_seq),
                _ => Err("Error"), // TODO: better erro
            }
        })
        .sum::<Result<usize, _>>()?
        .to_string())
}

fn dirpad(c: &char) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    match c {
        '^' => Ok((0, 1)),
        '>' => Ok((1, 2)),
        'v' => Ok((1, 1)),
        '<' => Ok((1, 0)),
        'A' => Ok((0, 2)),
        _ => Err("Invalid character".into()),
    }
}

fn numpad(c: &char) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    match c {
        '0' => Ok((3, 1)),
        '1' => Ok((2, 0)),
        '2' => Ok((2, 1)),
        '3' => Ok((2, 2)),
        '4' => Ok((1, 0)),
        '5' => Ok((1, 1)),
        '6' => Ok((1, 2)),
        '7' => Ok((0, 0)),
        '8' => Ok((0, 1)),
        '9' => Ok((0, 2)),
        'A' => Ok((3, 2)),
        _ => Err("Invalid character".into()),
    }
}

fn shortest_sequence(
    path: &str,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> Result<usize, Box<dyn std::error::Error>> {
    if let Some(length) = cache.get(&(path.to_string(), depth)) {
        return Ok(*length);
    }

    let pad = if depth == max_depth { numpad } else { dirpad };
    let gap = if depth == max_depth { (3, 0) } else { (0, 0) };

    let length = code_pairs(path)
        .iter()
        .map(|(c1, c2)| {
            let higher_paths = find_paths(pad(c1).unwrap(), pad(c2).unwrap(), gap); // TODO: avoid unwraps
            if depth == 1 {
                higher_paths.iter().map(String::len).min()
            } else {
                higher_paths
                    .iter()
                    .map(|p| shortest_sequence(p, depth - 1, max_depth, cache).unwrap())
                    .min()
            }
        })
        .map(|l| l.ok_or("No paths found".into()))
        .sum();

    if let Ok(l) = length {
        cache.insert((path.to_string(), depth), l);
    }
    length
}

fn find_paths(start: (usize, usize), end: (usize, usize), gap: (usize, usize)) -> Vec<String> {
    let mut paths = Vec::new();
    let mut q = VecDeque::from([(start, String::new())]);
    while let Some(((i, j), path)) = q.pop_front() {
        if (i, j) == end {
            let mut new_path = path.clone();
            new_path.push('A');
            paths.push(new_path);
            continue;
        }
        if i > end.0 && (i - 1, j) != gap {
            let mut new_path = path.clone();
            new_path.push('^');
            q.push_back(((i - 1, j), new_path));
        }
        if j < end.1 && (i, j + 1) != gap {
            let mut new_path = path.clone();
            new_path.push('>');
            q.push_back(((i, j + 1), new_path));
        }
        if i < end.0 && (i + 1, j) != gap {
            let mut new_path = path.clone();
            new_path.push('v');
            q.push_back(((i + 1, j), new_path));
        }
        if j > end.1 && (i, j - 1) != gap {
            let mut new_path = path.clone();
            new_path.push('<');
            q.push_back(((i, j - 1), new_path));
        }
    }

    paths
}

// TODO: return iterator / implement generic?
fn code_pairs(code: &str) -> Vec<(char, char)> {
    let cs = std::iter::once('A').chain(code.chars()).collect::<Vec<_>>();
    cs.iter()
        .zip(cs.iter().skip(1))
        .map(|(&a, &b)| (a, b))
        .collect()
}
