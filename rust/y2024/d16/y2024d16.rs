use std::{
    cmp::{min, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};
use strum::EnumIter;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (start, end, tracks) = parse_input(input)?;

    let mut dist = HashMap::new(); // Node -> cost
    let mut queue = BinaryHeap::new(); // [(Reverse(cost), Node)]

    let start_node = Node {
        position: start,
        direction: Direction::East,
    };
    queue.push((Reverse(0), start_node));
    dist.insert(start_node, 0);
    let mut best = u32::MAX;

    while let Some((Reverse(cost), node)) = queue.pop() {
        if cost >= best {
            break;
        }
        if node.position == end {
            best = min(best, cost);
            continue;
        }

        for (weight, neighbor) in node.neighbors() {
            if dist
                .get(&neighbor)
                .is_none_or(|&neighbor_cost| neighbor_cost >= cost)
                && tracks.contains(&neighbor.position)
            {
                let new_cost = cost + weight;
                queue.push((Reverse(new_cost), neighbor));
                dist.insert(neighbor, new_cost);
            }
        }
    }

    Ok(best.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Same as part1, but keep track of visited in all paths
    let (start, end, tracks) = parse_input(input)?;

    let mut dist = HashMap::new(); // Node -> cost
    let mut queue = BinaryHeap::new(); // [(Reverse(cost), Node, [visited])]
    let mut all_visited = HashSet::new(); // {position}

    let start_node = Node {
        position: start,
        direction: Direction::East,
    };
    queue.push((Reverse(0), start_node, vec![start]));
    dist.insert(start_node, 0);
    let mut best = u32::MAX;

    while let Some((Reverse(cost), node, visited)) = queue.pop() {
        if cost > best {
            break;
        }
        if node.position == end {
            best = min(best, cost);
            all_visited.extend(visited);
            continue;
        }

        for (weight, neighbor) in node.neighbors() {
            if dist
                .get(&neighbor)
                .is_none_or(|&neighbor_cost| neighbor_cost >= cost)
                && tracks.contains(&neighbor.position)
            {
                let new_cost = cost + weight;
                let mut new_visited = visited.clone();
                new_visited.push(neighbor.position);
                queue.push((Reverse(new_cost), neighbor, new_visited));
                dist.insert(neighbor, new_cost);
            }
        }
    }

    Ok(all_visited.len().to_string())
}
type Coord = (usize, usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    position: Coord,
    direction: Direction,
}

impl Node {
    fn move_forward(self) -> Node {
        let (x, y) = self.position;
        let new_coord = match self.direction {
            Direction::North => (x - 1, y),
            Direction::East => (x, y + 1),
            Direction::South => (x + 1, y),
            Direction::West => (x, y - 1),
        };
        Node {
            position: new_coord,
            direction: self.direction,
        }
    }

    fn turn_left(self) -> Node {
        Node {
            position: self.position,
            direction: self.direction.turn_left(),
        }
    }

    fn turn_right(self) -> Node {
        Node {
            position: self.position,
            direction: self.direction.turn_right(),
        }
    }

    fn neighbors(self) -> [(u32, Node); 3] {
        [
            (1, self.move_forward()),
            (1001, self.turn_left().move_forward()),
            (1001, self.turn_right().move_forward()),
        ]
    }
}

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
