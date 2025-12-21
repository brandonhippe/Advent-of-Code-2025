import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

from collections import deque
from functools import reduce
from itertools import chain, combinations
from operator import mul


rect_size = lambda cs: reduce(mul, map(lambda c: abs(c[0] - c[1]) + 1, zip(*cs)), 1)


def part1(data: List[str]) -> Any:
    """ 2025 Day 9 Part 1
    """
    return max(
        map(
            rect_size,
            combinations(map(lambda l: tuple(int(d) for d in re.findall(r"-?\d+", l)), data), 2)
        )
    )


def part2(data: List[str]) -> Any:
    """ 2025 Day 9 Part 2
    """
    corners = list(map(lambda l: tuple(int(d) for d in re.findall(r"-?\d+", l)), data))

    sorted_coords = list(
        map(
            lambda i: sorted(set(c[i] for c in corners)),
            range(len(corners[0]))
        )
    )
    reduced_corners = list(
        map(
            lambda c: tuple(
                sorted_coords[i].index(c[i]) for i in range(len(c))
            ),
            corners
        )
    )

    shape = set(chain.from_iterable(
        map(
            lambda c1_c2: (
                (x, y)
                for x in range(min(c1_c2[0][0], c1_c2[1][0]), max(c1_c2[0][0], c1_c2[1][0]) + 1)
                for y in range(min(c1_c2[0][1], c1_c2[1][1]), max(c1_c2[0][1], c1_c2[1][1]) + 1)
            ),
            zip(reduced_corners, reduced_corners[1:] + [reduced_corners[0]])
        ),
    ))
    
    to_visit = deque([
        next(
            filter(
                lambda c: c not in shape,
                map(
                    lambda c: tuple(c[i] + 1 for i in range(len(c))),
                    sorted(reduced_corners, key=lambda c: c[::-1])
                )
            )
        )
    ])
    while to_visit:
        x, y = to_visit.popleft()
        if (x, y) in shape:
            continue
        shape.add((x, y))
        for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            neighbor = (x + dx, y + dy)
            if neighbor not in shape:
                to_visit.append(neighbor)

    def rect_in_shape(corners: Tuple[Tuple[int, int], Tuple[int, int]]) -> bool:
        c1, c2 = corners
        x1, y1 = c1
        x2, y2 = c2
        rx1, ry1 = sorted_coords[0].index(x1), sorted_coords[1].index(y1)
        rx2, ry2 = sorted_coords[0].index(x2), sorted_coords[1].index(y2)

        corners = [(rx1, ry1), (rx1, ry2), (rx2, ry2), (rx2, ry1)]
        for c1, c2 in zip(corners, corners[1:] + [corners[0]]):
            for x in range(min(c1[0], c2[0]), max(c1[0], c2[0]) + 1):
                for y in range(min(c1[1], c2[1]), max(c1[1], c2[1]) + 1):
                    if (x, y) not in shape:
                        return False
        return True
    
    return max(
        map(
            rect_size,
            filter(
                rect_in_shape,
                combinations(corners, 2)
            )
        )
    )


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
