import argparse

from .run import run_solution


def main():
    parser = argparse.ArgumentParser("run a python solution")
    parser.add_argument("year", type=int)
    parser.add_argument("day", type=int)
    parser.add_argument(
        "--input",
        type=str,
        default=None,
        help="Path to input file. Defaults to input/y{year}d{day:02}.txt}",
        dest="input_path",
    )

    args = parser.parse_args()

    run_solution(args.year, args.day, args.input_path)


if __name__ == "__main__":
    main()
