# from collections import Counter


def part1(input_str: str) -> str:
    scanners = parse_input(input_str)
    # print(Counter(len(s) for s in scanners))
    return str(len(scanners))


def part2(input_str: str) -> str:
    raise NotImplementedError


type Scanner = list[tuple[int, int, int]]


def parse_input(input_str: str) -> list[Scanner]:
    scanners = []
    scanner: Scanner = []
    for line in input_str.splitlines():
        if line.startswith("--- scanner"):
            if scanner:
                scanners.append(scanner)
            scanner = []
        elif line == "":
            continue
        else:
            coords = tuple(map(int, line.split(",")))
            assert len(coords) == 3
            scanner.append(coords)
    scanners.append(scanner)

    return scanners
