import numpy as np  # type: ignore[import-not-found]
from scipy import optimize  # type: ignore[import-not-found]


def part1(input_str: str) -> str:
    raise NotImplementedError


def part2(input_str: str) -> str:
    machines = parse_input(input_str)
    tot = sum(m.find_joltage_presses() for m in machines)
    return str(tot)


class Machine:
    def __init__(self, buttons: list[list[int]], joltage: list[int]):
        self.buttons = buttons
        self.joltage = joltage

    @classmethod
    def from_str(cls, s: str) -> "Machine":
        (_, *buttons_str, joltage_str) = s.split(" ")
        buttons = [list(map(int, s.strip("()").split(","))) for s in buttons_str]
        joltage = list(
            map(int, joltage_str.removeprefix("{").removesuffix("}").split(","))
        )
        return Machine(buttons, joltage)

    def find_joltage_presses(self) -> int:
        n_vars = len(self.buttons)
        b = np.array(
            [[i in b for i in range(len(self.joltage))] for b in self.buttons]
        ).T
        j = np.array(self.joltage)
        res = optimize.milp(
            c=np.ones(n_vars),
            constraints=optimize.LinearConstraint(b, lb=j, ub=j),
            integrality=1,
        )
        return int(res.fun)


def parse_input(input_str: str) -> list[Machine]:
    return list(map(Machine.from_str, input_str.splitlines()))
