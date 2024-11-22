from dataclasses import dataclass
import re
from typing import Literal
from collections.abc import Sequence


def part1(input_str: str) -> str:
    instructions = parse_input(input_str)
    state: State = [Box("off", (-50, 50), (-50, 50), (-50, 50))]
    for i in instructions:
        state = execute(i, state)
    return str("")


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
    new_state = []
    for box in state:
        if box.state == instruction.state:
            new_state.append(box)
        else:
            new_state.extend(intersect(instruction, box))
    return []


def tuples_intersect(t1: tuple[int, int], t2: tuple[int, int]) -> bool:
    return (t1[0] < t2[0] < t1[1]) or (t2[0] < t1[0] < t1[0])


def intersect(i: Box, box: Box) -> Sequence[Box]:
    if not any(tuples_intersect(getattr(i, d), getattr(box, d)) for d in "xyz"):
        # no intersection, box stays as it is
        return box
    
    # todo: for each dimension, the instruction segment can
    # - not intersect (if any, there is no intersection)
    # - partially overlap (split the dimension in two, flip one)
    # - include the box (flip without splitting)
    # - be within the box (split in three, flip one)
