pub const RUST_TEMPLATE: &str =
    "pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 1
    Err(\"Solution not implemented\".into())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err(\"Solution not implemented\".into())
}
";

pub fn day_cargo_template(year: &i32, day: &u32) -> String {
    let name = format!("y{year}d{:02}", day);
    format!(
        "[package]
name = \"{name}\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
path = \"{name}.rs\"
"
    )
}

pub const PYTHON_TEMPLATE: &str = "def part1(input_str: str) -> str:
    raise NotImplementedError

def part2(input_str: str) -> str:
    raise NotImplementedError
";

pub fn haskell_template(year: &i32, day: &u32) -> String {
    format!(
        "module Year{year}.Day{:02} (part1, part2) where

part1 :: String -> String
part1 = error \"Part1 not implemented\"

part2 :: String -> String
part2 = error \"Part2 not implemented\"
",
        day
    )
}
