import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

from functools import reduce
from itertools import combinations
from operator import mul

from shapely import Polygon


def part1(data: List[str]) -> Any:
    """ 2025 Day 9 Part 1
    """
    return max(
        map(
            lambda cs: reduce(mul, map(lambda c: abs(c[0] - c[1]) + 1, zip(*cs)), 1),
            combinations(map(lambda l: tuple(int(d) for d in re.findall(r"-?\d+", l)), data), 2)
        )
    )


def part2(data: List[str]) -> Any:
    """ 2025 Day 9 Part 2
    """
    corners = list(map(lambda l: tuple(int(d) for d in re.findall(r"-?\d+", l)), data))
    boundary = Polygon(corners)

    def valid_rectangle(corners: Tuple[Tuple[int, int], Tuple[int, int]]) -> bool:
        c1, c2 = corners
        x1, y1 = c1
        x2, y2 = c2
        c3 = (x2, y1)
        c4 = (x1, y2)
        rect = Polygon([c1, c3, c2, c4])
        return rect.within(boundary)
    
    rect_size = lambda cs: reduce(mul, map(lambda c: abs(c[0] - c[1]) + 1, zip(*cs)), 1)
    return rect_size(next(
        filter(
            valid_rectangle,
            sorted(combinations(corners, 2), key=rect_size, reverse=True)
        )
    ))


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
        print(f"\nPart 1:\nArea of largest rectangle: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nArea of largest rectangle entirely within boundary: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
