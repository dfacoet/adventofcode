use std::collections::{BinaryHeap, HashMap, HashSet};
use strum::{EnumIter, IntoEnumIterator};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (start, end, tracks) = parse_input(input)?;

    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();

    let start_node = Node {
        position: start,
        direction: Direction::East,
    };
    dist.insert(start_node, 0);
    queue.push((0, start_node));

    while let Some((cost, node)) = queue.pop() {
        for (weight, neighbor) in node.neighbors() {
            if tracks.contains(&neighbor.position) {
                let new_cost = cost + weight;
                if dist
                    .get(&neighbor) // key not present OR condition
                    .is_none_or(|&current_cost| new_cost < current_cost)
                {
                    queue.push((new_cost, neighbor));
                    dist.insert(neighbor, new_cost); // TODO: decrease cost instead of inserting new!
                }
            }
        }
    }
    Direction::iter()
        .filter_map(|d| {
            dist.get(&Node {
                position: end,
                direction: d,
            })
        })
        .min()
        .map(|s| s.to_string())
        .ok_or("No path found".into())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
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
            (1000, self.turn_left()),
            (1000, self.turn_right()),
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
