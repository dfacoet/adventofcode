from collections import defaultdict
import graphlib


def part1(input_str: str) -> str:
    pairs, updates = parse_input(input_str)
    graph = build_graph(pairs)
    total = sum(
        update[len(update) // 2] for update in updates if is_sorted(update, graph)
    )
    return str(total)


def part2(input_str: str) -> str:
    pairs, updates = parse_input(input_str)
    graph = build_graph(pairs)
    total = sum(map(lambda x: get_middle_sorted(x, graph), updates))
    return str(total)


def parse_input(input_str: str) -> tuple[list[tuple[int, int]], list[list[int]]]:
    lines = input_str.splitlines()
    pairs = []
    while line := lines.pop(0):
        x, y = line.split("|")
        pairs.append((int(x), int(y)))

    updates = [list(map(int, line.split(","))) for line in lines]

    return pairs, updates


def build_graph(pairs: list[tuple[int, int]]) -> dict[int, list[int]]:
    graph: dict[int, list[int]] = defaultdict(list)
    for s, t in pairs:
        graph[s].append(t)
    return dict(graph)


def is_sorted(update: list[int], graph: dict[int, list[int]]):
    update_pos = {n: i for i, n in enumerate(update)}

    for n, successors in graph.items():
        if n not in update_pos:
            continue
        for m in successors:
            if update_pos[n] > update_pos.get(m, float("inf")):
                return False

    return True


def get_middle_sorted(update: list[int], graph: dict[int, list[int]]) -> int:
    if is_sorted(update, graph):
        return 0
    update_set = set(update)
    reduced_graph = {n: [m for m in graph[n] if m in update_set] for n in update}
    sorted_update = list(graphlib.TopologicalSorter(reduced_graph).static_order())
    return sorted_update[len(update) // 2]
