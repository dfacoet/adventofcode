pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (registers, program) = parse_input(input)?;
    let output = run_program(&registers, &program)?;
    Ok(output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let (_, program) = parse_input(input)?;
    let mut candidates = vec![0];
    // Find possible values of a iteratively: the possible values at step i
    // are obtained by adding 3 least significant bits to values at i-1,
    // and filtering for producing the correct last output values
    // NOTE: this works because (checked the input) the program is a finite
    // number of iterations, each of which prints a function of a and
    // gets rid of the 3 LSBs (instruction (0,3) a = a / 8)
    // and registers B and C are reset between iterations
    for i in 0..program.len() {
        candidates = candidates
            .iter()
            .flat_map(|a| {
                (8 * a..8 * a + 8).filter(|a| {
                    run_program(&(*a, 0, 0), &program).unwrap() == program[program.len() - i - 1..]
                })
            })
            .collect();
    }

    candidates
        .first()
        .map(|a| a.to_string())
        .ok_or("No solution found".into())
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

fn run_program(reg: &Registers, program: &[u64]) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let (mut a, mut b, mut c) = reg;
    let mut pointer = 0;
    let mut output = Vec::new();
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

    Ok(output)
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
