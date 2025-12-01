use std::fmt::Error;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut dial = Dial::new();
    for instruction in input.lines() {
        dial.execute1(instruction)?;
    }
    Ok(dial.count.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut dial = Dial::new();
    for instruction in input.lines() {
        dial.execute2(instruction)?;
    }
    Ok(dial.count.to_string())
}

// 5784 is not right
// 5157

struct Dial {
    position: i64,
    count: usize,
}

impl Dial {
    fn new() -> Self {
        Dial {
            position: 50,
            count: 0,
        }
    }

    fn rotate_left1(self: &mut Self, amount: i64) {
        self.position = (self.position - amount) % 100;
        if self.position == 0 {
            self.count += 1
        }
    }

    fn rotate_right1(self: &mut Self, amount: i64) {
        self.position = (self.position + amount) % 100;
        if self.position == 0 {
            self.count += 1
        }
    }

    fn execute1(self: &mut Self, instruction: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut chars = instruction.chars();
        let first = chars.next().expect("invalid instruction");
        let amount: i64 = chars.collect::<String>().parse()?;
        match first {
            'L' => self.rotate_left1(amount),
            'R' => self.rotate_right1(amount),
            _ => return Err("Invalid instruction".into()),
        }
        Ok(())
    }

    fn rotate_left2(self: &mut Self, amount: i64) {
        let new = self.position - amount;
        println!(
            "old {} new {} -> {}",
            self.position,
            new,
            new.rem_euclid(100)
        );
        if new < 0 {
            println!("adding {}", 1 + usize::try_from(-new).unwrap() / 100);
            self.count += usize::try_from(-new).unwrap() / 100;
            if self.position > 0 {
                println!("   +1");
                self.count += 1
            }
        }
        if new == 0 {
            println!("ended at 0 - adding 1");
            self.count += 1;
        }
        self.position = new.rem_euclid(100);
    }

    fn rotate_right2(self: &mut Self, amount: i64) {
        let new = self.position + amount;
        self.position = new.rem_euclid(100);
        self.count += new as usize / 100;
    }

    fn execute2(self: &mut Self, instruction: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut chars = instruction.chars();
        let first = chars.next().expect("invalid instruction");
        let amount: i64 = chars.collect::<String>().parse()?;
        match first {
            'L' => self.rotate_left2(amount),
            'R' => self.rotate_right2(amount),
            _ => return Err("Invalid instruction".into()),
        }
        Ok(())
    }
}
