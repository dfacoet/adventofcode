use std::collections::{HashMap, HashSet};

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let boxes = parse_input(input)?;
    match find_components(boxes, Some(1000)) {
        Components::Sizes(mut component_sizes) if component_sizes.len() >= 3 => {
            component_sizes.sort_by(|a, b| b.cmp(a));
            Ok(component_sizes
                .iter()
                .take(3)
                .product::<usize>()
                .to_string())
        }
        _ => Err("Solution not found".into()), // unreachable for valid input
    }
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let boxes = parse_input(input)?;
    match find_components(boxes, None) {
        Components::LastConnection(a, b) => Ok((a[0] * b[0]).to_string()),
        Components::Sizes(_) => Err("Solution not found".into()), // unreachable
    }
}

type Coord = [i64; 3];

fn parse_input(input: String) -> Result<Vec<Coord>, Box<dyn std::error::Error>> {
    let x = input
        .trim()
        .lines()
        .map(|line| {
            let values: Result<Vec<_>, _> = line.split(',').map(str::parse).collect();
            values.map(|v| [v[0], v[1], v[2]])
        })
        .collect::<Result<_, _>>()?;
    Ok(x)
}

fn d2(a: &Coord, b: &Coord) -> i64 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| (ai - bi).pow(2))
        .sum()
}

enum Components {
    LastConnection(Coord, Coord),
    Sizes(Vec<usize>),
}

fn find_components(boxes: Vec<Coord>, max_connections: Option<usize>) -> Components {
    let mut distances = Vec::new();
    for (i, x) in boxes.iter().enumerate() {
        for (j, y) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((d2(x, y), i, j));
        }
    }
    distances.sort();
    let max_connections = max_connections.unwrap_or(distances.len());

    let mut ids_to_component: HashMap<_, _> = (0..boxes.len()).map(|i| (i, i)).collect();
    let mut components: HashMap<_, _> = (0..boxes.len()).map(|i| (i, HashSet::from([i]))).collect();
    for (_, i, j) in distances.iter().take(max_connections) {
        let c1_id = *ids_to_component.get(i).unwrap();
        let c2_id = *ids_to_component.get(j).unwrap();
        if c1_id == c2_id {
            continue;
        }
        let component2 = components.remove(&c2_id).unwrap();
        for id in component2.iter() {
            ids_to_component.insert(*id, c1_id);
        }
        components
            .entry(c1_id)
            .or_insert_with(|| panic!())
            .extend(component2);

        if components.len() == 1 {
            return Components::LastConnection(boxes[*i], boxes[*j]);
        }
    }
    Components::Sizes(components.values().map(|c| c.len()).collect())
}
