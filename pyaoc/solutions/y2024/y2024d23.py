import itertools

import networkx as nx  # type: ignore[import-not-found]


def part1(input_str: str) -> str:
    g = nx.Graph()
    g.add_edges_from(parse_input(input_str))
    n = sum(
        1
        for c in itertools.takewhile(lambda x: len(x) <= 3, nx.enumerate_all_cliques(g))
        if len(c) == 3 and any(s.startswith("t") for s in c)
    )
    return str(n)


def part2(input_str: str) -> str:
    g = nx.Graph()
    g.add_edges_from(parse_input(input_str))
    max_clique = max(nx.find_cliques(g), key=len)
    return ",".join(sorted(max_clique))


def parse_input(input_str: str) -> list[list[str]]:
    return [s.split("-") for s in input_str.splitlines()]
