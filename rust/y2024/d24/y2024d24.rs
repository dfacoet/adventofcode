use std::collections::HashMap;

use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (mut values, wires) = parse_input(input);

    let mut output: Vec<_> = wires
        .keys()
        .filter_map(|name| {
            if name.starts_with("z") {
                Some((name, get_value(name, &mut values, &wires)))
            } else {
                None
            }
        })
        .collect();
    output.sort();

    Ok(output
        .iter()
        .enumerate()
        .map(|(i, (_, v))| 2_u32.pow(i as u32) * *v as u32)
        .sum::<u32>()
        .to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

struct Logic {
    gate: fn(bool, bool) -> bool,
    input1: String,
    input2: String,
    output: String,
}

impl FromStr for Logic {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 5 {
            return Err("Invalid logic string format".into());
        }
        let gate = match parts[1] {
            "AND" => |a, b| a && b,
            "OR" => |a, b| a || b,
            "XOR" => |a, b| a ^ b,
            _ => return Err("Invalid gate type".into()),
        };
        let logic = Logic {
            gate,
            input1: parts[0].to_string(),
            input2: parts[2].to_string(),
            output: parts[4].to_string(),
        };
        Ok(logic)
    }
}

fn parse_input(input: String) -> (HashMap<String, bool>, HashMap<String, Logic>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let (input_str, logic_str) = match parts.as_slice() {
        [input_str, logic_str] => (*input_str, *logic_str),
        _ => panic!("Invalid input format"),
    };
    let inputs = input_str
        .lines()
        .filter_map(|s| {
            s.split_once(": ").and_then(|(name, value)| match value {
                "0" => Some((name.to_string(), false)),
                "1" => Some((name.to_string(), true)),
                _ => None,
            })
        })
        .collect();
    let wires = logic_str
        .lines()
        .map(Logic::from_str)
        .map(|x| x.map(|x| (x.output.clone(), x)))
        .collect::<Result<HashMap<_, _>, _>>()
        .unwrap();

    (inputs, wires)
}

fn get_value(
    name: &str,
    values: &mut HashMap<String, bool>,
    wires: &HashMap<String, Logic>,
) -> bool {
    if let Some(v) = values.get(name) {
        return *v;
    }
    let w = wires.get(name).unwrap();
    let v = (w.gate)(
        get_value(&w.input1, values, wires),
        get_value(&w.input2, values, wires),
    );
    values.insert(name.to_string(), v);
    v
}
