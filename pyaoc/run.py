from typing import Callable

SolutionFn = Callable[[str], str]


def run_solution(year: int, day: int):
    with open(f"input/y{year}d{day:02}.txt") as f:
        input_string = f.read()

    # TODO: do this properly (importlib?)
    part1: SolutionFn
    part2: SolutionFn
    eval(f"from .solutions.y{year}.y{year}d{day:02} import part1, part2")

    print(f"year {year} day {day:02}")
    print("========================")
    try:
        sol1 = part1(input_string)  # noqa: F821
    except NotImplementedError:
        print("Part1 not implemented")
    else:
        print(f"Part1: {sol1}")

    try:
        sol2 = part2(input_string)  # noqa: F821
    except NotImplementedError:
        print("Part2 not implemented")
    else:
        print(f"Part2: {sol2}")
