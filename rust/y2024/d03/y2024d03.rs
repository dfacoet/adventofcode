use regex::Regex;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut total: u64 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    for mat in re.captures_iter(&input) {
        if let (Some(x), Some(y)) = (mat.get(1), mat.get(2)) {
            if let (Ok(x), Ok(y)) = (x.as_str().parse::<u64>(), y.as_str().parse::<u64>()) {
                total += x * y;
            }
        }
    }
    Ok(total.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut total: u64 = 0;
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?;
    for mat in re.captures_iter(&input) {
        match (mat.get(0).map(|m| m.as_str()), mat.get(1), mat.get(2)) {
            (Some(_), Some(x), Some(y)) => {
                if enabled {
                    if let (Ok(x), Ok(y)) = (x.as_str().parse::<u64>(), y.as_str().parse::<u64>()) {
                        total += x * y;
                    }
                }
            }
            (Some("do()"), None, None) => enabled = true,
            (Some("don't()"), None, None) => enabled = false,
            _ => panic!("impossible"),
        }
    }
    Ok(total.to_string())
}
