import itertools
import re
from collections.abc import Iterable
from dataclasses import dataclass
from typing import Literal


def part1(input_str: str) -> str:
    instructions = parse_input(input_str)
    state: State = [Box("off", (-50, 50), (-50, 50), (-50, 50))]
    for i in instructions:
        state = execute(i, state)
    return ""


def part2(input_str: str) -> str:
    raise NotImplementedError


@dataclass(frozen=True)
class Box:
    # Can represent both a (state) box or an instruction. Differentiate?
    state: Literal["on", "off"]
    x: tuple[int, int]
    y: tuple[int, int]
    z: tuple[int, int]


State = list[Box]


def parse_input(input_str: str) -> tuple[Box, ...]:
    return tuple(map(parse_line, input_str.splitlines()))


def parse_line(s: str) -> Box:
    pattern = re.compile(
        r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$"
    )
    match = pattern.match(s)
    if match:
        state, xmin, xmax, ymin, ymax, zmin, zmax = match.groups()
        return Box(
            state=state,  # type: ignore
            x=(int(xmin), int(xmax)),
            y=(int(ymin), int(ymax)),
            z=(int(zmin), int(zmax)),
        )
    else:
        raise ValueError(f"Line does not match expected format: {s}")


def execute(instruction: Box, state: State) -> State:
    return list(
        itertools.chain.from_iterable(intersect(instruction, box) for box in state)
    )


def intersect(i: Box, box: Box) -> Iterable[Box]:
    boxes = [box]
    for _dim in "xyz":
        new_boxes: list[Box] = []
        for _box in boxes:
            # compare box.getattr(dim) and i.getattr(dim)
            pass
    return new_boxes
