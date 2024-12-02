def part1(input_str: str) -> str:
    all_levels = parse_input(input_str)
    return str(sum(1 for levels in all_levels if is_safe(levels)))


def part2(input_str: str) -> str:
    all_levels = parse_input(input_str)
    return str(sum(1 for levels in all_levels if is_almost_safe(levels)))


def parse_input(input_str: str) -> list[list[int]]:
    return [[int(x) for x in line.split()] for line in input_str.splitlines()]


def is_safe(levels) -> bool:
    diffs = {b - a for a, b in zip(levels, levels[1:])}
    return diffs.issubset({1, 2, 3}) or diffs.issubset({-1, -2, -3})


def is_almost_safe(levels) -> bool:
    return any(is_safe(levels[:k] + levels[k + 1 :]) for k in range(len(levels)))
