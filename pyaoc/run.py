import importlib
from typing import Callable

SolutionFn = Callable[[str], str]


class SolutionNotFound(Exception):
    pass


def run_solution(year: int, day: int):
    with open(f"input/y{year}d{day:02}.txt") as f:
        input_string = f.read()

    module_name = f"y{year}d{day:02}"
    try:
        solution_module = importlib.import_module(
            f"pyaoc.solutions.y{year}." + module_name
        )
    except ModuleNotFoundError as e:
        raise ValueError(f"No solution module found for {module_name}") from e

    print(f"year {year} day {day:02}")
    print("================")
    try:
        part1: SolutionFn = getattr(solution_module, "part1")
        sol1 = part1(input_string)
    except (AttributeError, NotImplementedError):
        print("Part1 not implemented")
    except SolutionNotFound:
        print("Part1: solution not found")
    else:
        print(f"Part1: {sol1}")

    try:
        part2: SolutionFn = getattr(solution_module, "part2")
        sol2 = part2(input_string)
    except (AttributeError, NotImplementedError):
        print("Part2 not implemented")
    except SolutionNotFound:
        print("Part2: solution not found")
    else:
        print(f"Part2: {sol2}")
