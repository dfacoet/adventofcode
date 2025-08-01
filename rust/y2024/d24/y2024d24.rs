use std::collections::{HashMap, HashSet};

use std::hash::Hash;
use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (input, wires) = parse_input(input);
    let output = solve(&input, &wires);
    Ok(output.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Not a general solution, but
    // - assume the correct implementation is a ripple-carry adder
    // - we only need to find 8 wrong outputs (and don't need to match them)
    // - assume each swap involves two gates of different kind, so validation
    //   is done at a local level (checking only the labels and number of connections)
    let (input, wires) = parse_input(input);
    let n_bits = validate_input(&input, &wires)?;

    // TODO: better way to fold and create the new wires map in one go?
    let output_connection_counts: HashMap<String, usize> =
        wires.values().fold(HashMap::new(), |mut acc, logic| {
            *acc.entry(logic.input1.clone()).or_insert(0) += 1; // avoid cloning?
            *acc.entry(logic.input2.clone()).or_insert(0) += 1;
            if logic.output.starts_with('z') {
                *acc.entry(logic.output.clone()).or_insert(0) += 0;
            }
            acc
        });
    let wires: HashMap<String, (Logic, usize)> = wires // output name -> (Logic, n_output_connections)
        .into_iter()
        .map(|(k, v)| {
            let count = output_connection_counts.get(&k).unwrap();
            (k, (v, *count))
        })
        .collect();

    let mut wrong_outputs: Vec<_> = wires
        .iter()
        .filter_map(|(output, (logic, n_output_connections))| {
            if is_wrong_logic(logic, n_output_connections, &n_bits) {
                Some(output.to_string())
            } else {
                None
            }
        })
        .collect();

    assert_eq!(wrong_outputs.len(), 8);
    wrong_outputs.sort();
    Ok(wrong_outputs.join(","))
}

fn is_wrong_logic(logic: &Logic, n_output_connections: &usize, n_bits: &usize) -> bool {
    match logic.gate {
        Gate::Xor => {
            !logic.output.starts_with("z")
                && (*n_output_connections != 2
                    || !(logic.input1.starts_with("x") || logic.input2.starts_with("x")))
        }
        Gate::And => logic.input1 != "x00" && logic.input2 != "x00" && *n_output_connections != 1,
        Gate::Or => logic.output != format!("z{n_bits}") && *n_output_connections != 2,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

impl FromStr for Gate {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Gate::And),
            "OR" => Ok(Gate::Or),
            "XOR" => Ok(Gate::Xor),
            _ => Err("Invalid gate type".into()),
        }
    }
}

impl Gate {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::And => a && b,
            Gate::Or => a || b,
            Gate::Xor => a ^ b,
        }
    }
}

struct Logic {
    gate: Gate,
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
        let gate = Gate::from_str(parts[1])?;
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
    let v = w.gate.apply(
        get_value(&w.input1, values, wires),
        get_value(&w.input2, values, wires),
    );
    values.insert(name.to_string(), v);
    v
}

fn solve(input: &HashMap<String, bool>, wires: &HashMap<String, Logic>) -> u64 {
    let mut values = input.clone();
    wires
        .keys()
        .filter(|name| name.starts_with("z"))
        .for_each(|name| {
            get_value(name, &mut values, wires);
        });
    get_n('z', &values)
}

fn get_n(prefix: char, values: &HashMap<String, bool>) -> u64 {
    let mut n: Vec<_> = values
        .iter()
        .filter(|(name, _)| name.starts_with(prefix))
        .collect();
    n.sort();
    n.iter()
        .enumerate()
        .map(|(i, (_, v))| 2_u64.pow(i as u32) * **v as u64)
        .sum::<u64>()
}

fn validate_input(
    input: &HashMap<String, bool>,
    wires: &HashMap<String, Logic>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let n_bits = input.len() / 2;

    // Check that inputs and output have the expected number of bits and labels
    let xs: HashSet<_> = input.keys().filter(|s| s.starts_with('x')).collect();
    let ys: HashSet<_> = input.keys().filter(|s| s.starts_with('y')).collect();
    let zs: HashSet<_> = wires.keys().filter(|s| s.starts_with('z')).collect();
    assert_eq!(xs.len(), n_bits);
    assert_eq!(ys.len(), n_bits);
    assert_eq!(zs.len(), n_bits + 1); // Output has one extra bit
    let width = (n_bits - 1).to_string().len();
    assert!(zs.contains(&format!("z{}", n_bits)));
    for i in 0..n_bits {
        assert!(xs.contains(&format!("x{i:0width$}")));
        assert!(ys.contains(&format!("y{i:0width$}")));
        assert!(zs.contains(&format!("z{i:0width$}")));
    }

    // Check that the gates are those expected for a ripple-carry adder with n_bits
    let gate_counts = wires.values().fold(HashMap::new(), |mut acc, w| {
        *acc.entry(w.gate).or_insert(0) += 1;
        acc
    });
    // For each bit, we have two AND, two XOR and one OR gate
    // except for the zero-th bit, which only has one AND and one XOR
    assert_eq!(gate_counts.get(&Gate::And), Some(&(2 * n_bits - 1)));
    assert_eq!(gate_counts.get(&Gate::Xor), Some(&(2 * n_bits - 1)));
    assert_eq!(gate_counts.get(&Gate::Or), Some(&(n_bits - 1)));

    Ok(n_bits)
}
