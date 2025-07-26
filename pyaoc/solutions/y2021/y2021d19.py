import itertools
from collections import Counter, deque
from collections.abc import Iterable
from dataclasses import dataclass
from functools import cache, reduce
from operator import mul


def part1(input_str: str) -> str:
    sol, _ = solve(input_str)

    return str(sol)


def part2(input_str: str) -> str:
    _, sol = solve(input_str)

    return str(sol)


@cache
def solve(input_str: str) -> tuple[int, int]:
    scanners = parse_input(input_str)

    known_beacons = set(scanners[0])
    queue = deque(scanners[1:])
    scanner_coords: list[Coord] = [(0, 0, 0)]

    # Scanners in the queue have unknown coordinates. If overlap is found, remove it
    # from the queue, and all its beacons become known. Iterate until all beacon and
    # scanner coordinates are found.
    # Improvement: keep track of scanners in the last level, and check for overlap
    # only with them (caching reduces the need for this improvement).
    while queue:
        new_beacons = queue.popleft()
        match find_overlap(known_beacons, new_beacons):
            case AffineTransformation() as t:
                known_beacons.update(map(t, new_beacons))
                scanner_coords.append(t.translation)
            case None:
                queue.append(new_beacons)

    max_dist = max(l1(x, y) for x, y in itertools.combinations(scanner_coords, 2))
    return len(known_beacons), max_dist


type Coord = tuple[int, int, int]
type Beacons = Iterable[Coord]
type DistCounts = dict[int, int]


# Some type ignores because mypy can't always count
# Would be easier and faster with arrays, but I'll stick to pure python


@dataclass
class LinearTransformation:
    perm: tuple[int, int, int]  # permutation of [0,1,2]
    reflection: tuple[int, int, int]  # \pm 1

    def __call__(self, x: Coord) -> Coord:
        tx = tuple(r * x[i] for i, r in zip(self.perm, self.reflection))
        return tx  # type: ignore[return-value]

    def is_rotation(self) -> bool:
        perm_sign = 1 if self.perm in {(0, 1, 2), (1, 2, 0), (2, 0, 1)} else -1
        refl_sign = reduce(mul, self.reflection)
        return perm_sign * refl_sign > 0


@dataclass
class AffineTransformation(LinearTransformation):
    translation: tuple[int, int, int]

    def __call__(self, x: Coord) -> Coord:
        tx = tuple(
            r * x[i] + d
            for i, r, d in zip(self.perm, self.reflection, self.translation)
        )
        return tx  # type: ignore[return-value]


ROTATIONS = [
    t
    for p in itertools.permutations((0, 1, 2))
    for r in itertools.product((1, -1), repeat=3)
    if (t := LinearTransformation(perm=p, reflection=tuple(r))).is_rotation()  # type: ignore[arg-type]
]


@cache
def delta(x: Coord, y: Coord) -> tuple[int, int, int]:
    # return (vector) position difference
    return tuple(i - j for i, j in zip(x, y))  #  type: ignore[return-value]


@cache
def l1(x: Coord, y: Coord) -> int:
    return sum(map(abs, delta(x, y)))


def count_distances(xs: Beacons) -> dict[Coord, DistCounts]:
    # Within a group, compute all relative l1 distances and return them as
    # beacon coords -> distance -> count
    return {x: Counter(l1(x, y) for y in xs if y != x) for x in xs}


def count_overlap_distances(counts1: DistCounts, counts2: DistCounts):
    return sum(min(counts1[d], counts2[d]) for d in counts1.keys() & counts2.keys())


def find_coord_transformation(pairs: list[tuple[Coord, Coord]]) -> AffineTransformation:
    delta1 = delta(pairs[0][0], pairs[1][0])
    delta2 = delta(pairs[0][1], pairs[1][1])
    try:
        # TODO: there's a better way to find the parameters than looping through all
        r = next(t for t in ROTATIONS if t(delta2) == delta1)
    except StopIteration:
        raise RuntimeError("Transformation not found") from StopIteration

    offset = delta(pairs[0][0], r(pairs[0][1]))
    t = AffineTransformation(perm=r.perm, reflection=r.reflection, translation=offset)
    # assert all(t(y) == x for x, y in pairs)
    return t


def find_overlap(b1: Beacons, b2: Beacons) -> AffineTransformation | None:
    dcounts1, dcounts2 = map(count_distances, (b1, b2))

    # x and y are the same beacon iff they are both part of a group of 12 overlapping
    # beacons. Relative distances are frame-independent, so they must have the same
    # distances to at least 11 beacons in the respective groups.
    matching_beacons = [
        (x, y)
        for x, dx in dcounts1.items()
        for y, dy in dcounts2.items()
        if count_overlap_distances(dx, dy) >= 11
    ]

    if not matching_beacons:
        return None
    assert len(matching_beacons) >= 12

    return find_coord_transformation(matching_beacons)


def parse_input(input_str: str) -> list[list[Coord]]:
    scanners = []
    beacon_coords: list[Coord] = []
    for line in input_str.splitlines():
        if line.startswith("--- scanner"):
            if beacon_coords:
                scanners.append(beacon_coords)
                beacon_coords = []
        elif line == "":
            continue
        else:
            coords = tuple(map(int, line.split(",")))
            assert len(coords) == 3
            beacon_coords.append(coords)
    scanners.append(beacon_coords)

    return scanners
