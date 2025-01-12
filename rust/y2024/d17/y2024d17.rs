pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let ((mut a, mut b, mut c), program) = parse_input(input)?;
    let mut pointer = 0;
    let mut output: Vec<u64> = Vec::new();
    while pointer < program.len() {
        let operand = program[pointer + 1];
        match program[pointer] {
            0 => a = a / 2u64.pow(combo_operand_value(&operand, &a, &b, &c)? as u32),
            1 => b ^= operand,
            2 => b = combo_operand_value(&operand, &a, &b, &c)? % 8,
            3 => {
                if a != 0 && pointer != operand as usize {
                    pointer = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push(combo_operand_value(&operand, &a, &b, &c)? % 8),
            6 => b = a / 2u64.pow(combo_operand_value(&operand, &a, &b, &c)? as u32),
            7 => c = a / 2u64.pow(combo_operand_value(&operand, &a, &b, &c)? as u32),
            _ => return Err("Invalid opcode".into()),
        }
        pointer += 2;
    }
    Ok(output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

pub fn part2(_input: String) -> Result<String, Box<dyn std::error::Error>> {
    // Solve part 2
    Err("Solution not implemented".into())
}

type Registers = (u64, u64, u64);

fn parse_input(input: String) -> Result<(Registers, Vec<u64>), Box<dyn std::error::Error>> {
    let mut lines = input.lines();

    let reg_a = lines
        .next()
        .ok_or("Missing Register A")?
        .split(": ")
        .nth(1)
        .ok_or("Invalid Register A")?
        .parse::<u64>()?;
    let reg_b = lines
        .next()
        .ok_or("Missing Register B")?
        .split(": ")
        .nth(1)
        .ok_or("Invalid Register B")?
        .parse::<u64>()?;
    let reg_c = lines
        .next()
        .ok_or("Missing Register C")?
        .split(": ")
        .nth(1)
        .ok_or("Invalid Register C")?
        .parse::<u64>()?;
    lines.next();
    let program = lines
        .next()
        .ok_or("Missing Program")?
        .split(": ")
        .nth(1)
        .ok_or("Invalid Program")?
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    Ok(((reg_a, reg_b, reg_c), program))
}

fn combo_operand_value(
    operand: &u64,
    a: &u64,
    b: &u64,
    c: &u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    match *operand {
        x if x < 4 => Ok(x),
        4 => Ok(*a),
        5 => Ok(*b),
        6 => Ok(*c),
        _ => Err("Invalid operand".into()),
    }
}
