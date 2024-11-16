from pyaoc import SolutionNotFound


def part1(input_str: str) -> str:
    value = {"(": 1, ")": -1}
    sol = sum(value[c] for c in input_str)
    return str(sol)


def part2(input_str: str) -> str:
    value = {"(": 1, ")": -1}
    floor = 0
    for k, c in enumerate(input_str):
        floor += value[c]
        if floor < 0:
            return str(k + 1)
    raise SolutionNotFound
