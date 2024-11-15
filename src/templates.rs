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

[dependencies]
aoc = \"*\"
"
    )
}
