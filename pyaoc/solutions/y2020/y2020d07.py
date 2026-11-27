from collections import defaultdict
from dataclasses import dataclass


def part1(input_str: str) -> str:
    graph = parse_input(input_str)
    # build a "reversed" tree ("is contained by")
    contained_graph: defaultdict[Bag, set[Bag]] = defaultdict(set)
    for k, v in graph.items():
        for c in v:
            contained_graph[c.bag].add(k)
    start = Bag("shiny", "gold")
    queue = {start}
    visited = set()
    while queue:
        next_bag = queue.pop()
        visited.add(next_bag)
        queue.update(contained_graph[next_bag])
    return str(len(visited) - 1)


def part2(input_str: str) -> str:
    graph = parse_input(input_str)
    start = Bag("shiny", "gold")
    c = count(start, graph) - 1
    return str(c)

    # 6007 is too high


@dataclass(frozen=True)
class Bag:
    adj: str
    color: str


@dataclass(frozen=True)
class Content:
    bag: Bag
    n: int


type Rules = dict[Bag, list[Content]]


def parse_input(input_str: str) -> Rules:
    return dict(parse_line(s) for s in input_str.splitlines())


def parse_line(s: str) -> tuple[Bag, list[Content]]:
    match s.split(maxsplit=4):
        case [t, color, "bags", "contain", rest]:
            bag = Bag(t, color)
            contents = [
                c for s in rest.split(", ") if (c := parse_content(s)) is not None
            ]
            return (bag, contents)
        case _:
            raise ValueError(f"Invalid string: {s}")


def parse_content(s: str) -> Content | None:
    match s.split():
        case ["no", *_]:
            return None
        case [n, t, color, *_]:
            return Content(Bag(t, color), int(n))
        case _:
            raise ValueError(f"Invalid string: {s}")


def count(bag: Bag, graph: Rules) -> int:
    return 1 + sum(c.n * count(c.bag, graph) for c in graph[bag])
