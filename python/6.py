import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from operator import add, mul
from functools import reduce

operators = {"+": add, "*": mul}

def part1(data: List[str]) -> Any:
    """ 2025 Day 6 Part 1
    >>> part1(["123 328  51 64 ", " 45 64  387 23 ", "  6 98  215 314", "*   +   *   +  "])
    4277556
    """
    return sum(
        reduce(
            operators[p[-1]], 
            map(int, p[1:-1]), 
            int(p[0])
        ) for p in map(list, zip(
            *map(
                lambda l: filter(None, re.split(r"\s+", l)),
                data
            ), strict=True)
        )
    )


def part2(data: List[str]) -> Any:
    """ 2025 Day 6 Part 2
    >>> part2(["123 328  51 64 ", " 45 64  387 23 ", "  6 98  215 314", "*   +   *   +  "])
    3263827
    """
    ops = data.pop()
    op_ixs = list(map(lambda e: e[0], filter(lambda e: e[1] != " ", enumerate(ops)))) + [len(ops)]
    slices = list(slice(op_ixs[ix-1], op_ixs[ix]) for ix in range(1, len(op_ixs)))
    problems = map(lambda p: map(int, filter(None, map(lambda ns: ''.join(filter(lambda c: c != " ", ns)), zip(*p, strict=True)))), zip(*map(lambda l: map(lambda s: l[s], slices), data), strict=True))

    return sum(map(lambda args: reduce(*args), zip(map(lambda o: operators[o], filter(None, re.split(r"\s+", ops))), problems, strict=True)))


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nTotal of problem answers: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal of problem answers: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
