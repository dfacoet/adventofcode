use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let problems = parse_input(input)?;
    Ok(problems
        .iter()
        .filter_map(naive_solver)
        .sum::<u64>()
        .to_string())
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

#[derive(Debug)]
struct Problem {
    d_a: (u64, u64),
    d_b: (u64, u64),
    target: (u64, u64),
}

impl std::str::FromStr for Problem {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let parse_coords = |line: &str| -> Result<(u64, u64), Box<dyn std::error::Error>> {
            let parts: Vec<&str> = line.split(&[',', ' ', ':', '+', '='][..]).collect();
            let x = parts.iter().position(|&s| s == "X").ok_or("Missing X")?;
            let y = parts.iter().position(|&s| s == "Y").ok_or("Missing Y")?;
            Ok((parts[x + 1].parse()?, parts[y + 1].parse()?))
        };

        Ok(Problem {
            d_a: parse_coords(lines.next().ok_or("Missing Button A")?)?,
            d_b: parse_coords(lines.next().ok_or("Missing Button B")?)?,
            target: parse_coords(lines.next().ok_or("Missing Prize")?)?,
        })
    }
}

impl Problem {
    fn on_target(&self, (p_a, p_b): (u64, u64)) -> bool {
        (p_a * self.d_a.0 + p_b * self.d_b.0 == self.target.0)
            && (p_a * self.d_a.1 + p_b * self.d_b.1 == self.target.1)
    }
}

fn parse_input(input: String) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
    input.split("\n\n").map(Problem::from_str).collect()
}

fn naive_solver(p: &Problem) -> Option<u64> {
    let mut solutions = Vec::new();
    println!("{:?}", p);
    // TODO: use better limits
    for b in 0..101 {
        for a in 0..101 {
            if p.on_target((a, b)) {
                println!("{a},{b} is a solution");
                solutions.push((a, b));
            }
        }
    }
    solutions.iter().map(|(a, b)| 3 * a + b).min()
}
