import argparse

from .run import run_solution


def main():
    parser = argparse.ArgumentParser("run a python solution")
    parser.add_argument("year", type=int)
    parser.add_argument("day", type=int)

    args = parser.parse_args()

    run_solution(args.year, args.day)


if __name__ == "__main__":
    main()
