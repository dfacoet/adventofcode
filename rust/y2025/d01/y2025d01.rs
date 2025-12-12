pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut dial = Dial::new();
    for instruction in input.lines() {
        dial.rotate(parse_instruction(instruction)?)?;
    }
    Ok(dial.count_final.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut dial = Dial::new();
    for instruction in input.lines() {
        dial.rotate(parse_instruction(instruction)?)?;
    }
    Ok(dial.count_pass.to_string())
}

fn parse_instruction(instruction: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut chars = instruction.chars();
    let sign = match chars.next() {
        Some('R') => 1,
        Some('L') => -1,
        _ => return Err("Invalid instruction".into()),
    };
    let amount: i64 = chars.collect::<String>().parse()?;
    Ok(sign * amount)
}

struct Dial {
    position: i64,
    count_final: u64,
    count_pass: u64,
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            count_final: 0,
            count_pass: 0,
        }
    }

    fn rotate(&mut self, amount: i64) -> Result<(), Box<dyn std::error::Error>> {
        let new = self.position + amount;
        self.count_pass += u64::try_from((new / 100).abs())?;

        if new <= 0 && self.position > 0 {
            // Passed zero turning left
            self.count_pass += 1;
        }

        self.position = new.rem_euclid(100);
        if self.position == 0 {
            self.count_final += 1
        }
        Ok(())
    }
}
