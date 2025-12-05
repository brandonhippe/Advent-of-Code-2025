import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer


def part1(data: List[str]) -> Any:
    """ 2025 Day 5 Part 1
    """
    ranges, ingredients = "\n".join(data).split("\n\n")
    ranges = set(tuple(map(int, re.findall(r"\d+", r))) for r in ranges.splitlines())
    return len(list(filter(lambda ing: any(r[0] <= ing <= r[1] for r in ranges), map(int, ingredients.splitlines()))))

def part2(data: List[str]) -> Any:
    """ 2025 Day 5 Part 2
    """
    ranges, ingredients = "\n".join(data).split("\n\n")
    ranges = list(tuple(map(int, re.findall(r"\d+", r))) for r in ranges.splitlines())
    non_overlapping = []
    stack = sorted(ranges, reverse=True)
    while stack:
        r = stack.pop()
        assert len(list(map(lambda e: e[0], filter(lambda e: r[0] <= e[1][1], enumerate(non_overlapping))))) <= 1
        if non_overlapping and r[0] <= non_overlapping[-1][1]:
            last = non_overlapping.pop()
            non_overlapping.append((last[0], max(r[0] - 1, last[0])))
            stack.append((last[0] + 1, max(r[1], last[1])))
        else:
            non_overlapping.append(r)

    return sum(map(lambda r: r[1] - r[0] + 1, non_overlapping))


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
        print(f"\nPart 1:\nTotal fresh ingredients: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of possible fresh ingredients: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
