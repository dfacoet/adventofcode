from dataclasses import dataclass
import operator
from typing import Callable, Literal
import tqdm


def part1(input_str: str) -> str:
    instructions = parse_input(input_str)
    print("# instr:", len(instructions))

    for n in tqdm.trange(99_999_999_999_999, 11_111_111_111_111, -1, mininterval=1):
        digits = [int(c) for c in str(n)]
        if 0 in digits:
            continue
        vars = Vars(0, 0, 0, 0)
        for i in instructions:
            i.apply(vars, digits)
        assert not digits
        if vars.z == 0:
            return str(n)

    raise RuntimeError("Solution not found")


def part2(input_str: str) -> str:
    raise NotImplementedError


@dataclass
class Vars:
    w: int
    x: int
    y: int
    z: int


OP_MAP: dict[str, Callable[[int, int], int]] = {
    "add": operator.add,
    "mul": operator.mul,
    "div": operator.floordiv,  # TODO: fix on negatives
    "mod": operator.mod,
    "eql": operator.eq,
}


class Instruction:
    def apply(self, vars: Vars, digits: list[int]) -> None: ...

    @staticmethod
    def from_str(s: str) -> "Instruction | None":
        match s.split():
            case ("inp", "w"):
                return GetDigit()
            case ("div", _, "1"):  # div 1 is no-op
                return None
            case (op, a, b) if b in "wxyz":
                return BinOp(op=OP_MAP[op], a=a, b=b)  # type: ignore
            case (op, a, b):
                return UnOp(op=OP_MAP[op], a=a, b=int(b))  # type: ignore
            case _:
                raise ValueError("Invalid instruction:", s)


class GetDigit(Instruction):
    def apply(self, vars: Vars, digits: list[int]) -> None:
        vars.w = digits.pop(0)


@dataclass(frozen=True)
class UnOp(Instruction):
    op: Callable[[int, int], int]
    a: Literal["w", "x", "y", "z"]
    b: int

    def apply(self, vars: Vars, _: list[int]) -> None:
        r = self.op(getattr(vars, self.a), self.b)
        setattr(vars, self.a, r)


@dataclass(frozen=True)
class BinOp(Instruction):
    op: Callable[[int, int], int]
    a: Literal["w", "x", "y", "z"]
    b: Literal["w", "x", "y", "z"]

    def apply(self, vars: Vars, _: list[int]) -> None:
        r = self.op(getattr(vars, self.a), getattr(vars, self.b))
        setattr(vars, self.a, r)


def parse_input(input_str: str) -> list[Instruction]:
    return [
        i
        for line in input_str.splitlines()
        if (i := Instruction.from_str(line)) is not None
    ]


# Improvements:
# - w is only used to get input, never set. Remove.
# - check `div` operation (suspect not actually needed)
# - ops that are just setting value (e.g. mul x 0)
