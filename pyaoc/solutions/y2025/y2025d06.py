from functools import reduce
from operator import mul


def part1(input_str: str) -> str:
    *lines, ops_str = input_str.splitlines()
    ops = map({"+": sum, "*": product}.__getitem__, ops_str.split())
    nss = zip(*[map(int, s.split()) for s in lines])
    result = sum(op(ns) for op, ns in zip(ops, nss))
    return str(result)


def part2(input_str: str) -> str:
    *lines, ops_str = input_str.splitlines()
    ops = map({"+": sum, "*": product}.__getitem__, ops_str.split())
    cols = ["".join(s) for s in zip(*lines)]
    nss: list[list[int]] = [[]]
    for c in cols:
        try:
            nss[-1].append(int(c))
        except ValueError:
            nss.append([])
    result = sum(op(ns) for op, ns in zip(ops, nss))
    return str(result)


def product(xs):
    return reduce(mul, xs)
