pub fn part1(input: String) -> String {
    input
        .chars()
        .map(|p| match p {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum::<i64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut floor = 0;
    for (k, p) in input.chars().enumerate() {
        match p {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("!"),
        }
        if floor < 0 {
            return (k + 1).to_string();
        }
    }
    panic!("-1 not found")
}
