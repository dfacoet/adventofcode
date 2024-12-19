use std::collections::HashSet;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (mut pos, obstacles, mut boxes, moves) = parse_input(input)?;
    for m in moves {
        let mut c = next(pos, m);
        let mut box_to = None;
        while boxes.contains(&c) {
            c = next(c, m);
            box_to = Some(c);
        }
        if !obstacles.contains(&c) {
            pos = next(pos, m);
            if let Some(box_to) = box_to {
                boxes.remove(&pos);
                boxes.insert(box_to);
            }
        }
    }

    Ok(boxes
        .into_iter()
        .map(|(i, j)| 100 * i + j)
        .sum::<usize>()
        .to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

type Coord = (usize, usize);
type Input = (Coord, HashSet<Coord>, HashSet<Coord>, Vec<char>);

fn parse_input(input: String) -> Result<Input, Box<dyn std::error::Error>> {
    let v = input.split("\n\n").collect::<Vec<_>>();
    let (grid, moves) = if v.len() == 2 {
        (v[0], v[1])
    } else {
        return Err("Invalid input".into());
    };
    let obstacles = get_positions(grid, '#');
    let boxes = get_positions(grid, 'O');
    let robot_positions = get_positions(grid, '@');
    let robot = if robot_positions.len() == 1 {
        *robot_positions.iter().next().unwrap()
    } else {
        return Err("There should be exactly one robot position".into());
    };
    let moves: Vec<_> = moves.lines().collect::<String>().chars().collect(); // remove newlines
    let move_chars: HashSet<_> = "^v<>".chars().collect();
    if !moves.iter().all(|c| move_chars.contains(c)) {
        return Err("Invalid character in moves".into());
    };
    Ok((robot, obstacles, boxes, moves))
}

fn get_positions(grid: &str, ch: char) -> HashSet<Coord> {
    grid.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .filter_map(move |(j, c)| if c == ch { Some((i, j)) } else { None })
        })
        .collect()
}

fn next((i, j): Coord, m: char) -> Coord {
    match m {
        '^' => (i - 1, j),
        '>' => (i, j + 1),
        'v' => (i + 1, j),
        '<' => (i, j - 1),
        _ => panic!("Invalid move"),
    }
}
