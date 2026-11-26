def part1(input_str: str) -> str:
    program = parse_input(input_str)
    output = run(program, 12, 2)
    return str(output)


def part2(input_str: str) -> str:
    program = parse_input(input_str)
    for noun in range(100):
        for verb in range(100):
            if run(program, noun, verb) == 19690720:
                return str(100 * noun + verb)
    raise ValueError("Solution not found")


def parse_input(input_str) -> list[int]:
    return [int(x) for x in input_str.split(",")]


def run(program: list[int], noun: int, verb: int) -> int:
    state = program.copy()
    state[1:3] = [noun, verb]
    for k in range(0, len(state), 4):
        k1, k2, t = state[k + 1 : k + 4]
        match state[k]:
            case 1:
                state[t] = state[k1] + state[k2]
            case 2:
                state[t] = state[k1] * state[k2]
            case 99:
                return state[0]
            case _:
                raise ValueError("Invalid program")
    else:
        raise ValueError("Invalid program")
