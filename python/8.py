import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

import heapq
from functools import reduce
from itertools import combinations
from operator import mul


def part1(data: List[str]) -> Any:
    """ 2025 Day 8 Part 1
    """
    points = list(map(lambda l: tuple(map(int, re.findall(r'\d+', l))), data))
    distances = [(sum(map(lambda cs: (cs[0] - cs[1]) ** 2, zip(p1, p2))), p1, p2) for p1, p2 in combinations(points, 2) if p1 != p2]
    heapq.heapify(distances)

    circuits: List[set[Tuple[int,]]] = []
    for _ in range(1000):
        dist, p1, p2 = heapq.heappop(distances)
        p1_in = next(map(lambda it: circuits.pop(it[0]), filter(lambda it: p1 in it[1], enumerate(circuits))), {p1})
        p2_in = next(map(lambda it: circuits.pop(it[0]), filter(lambda it: p2 in it[1], enumerate(circuits))), {p2})
        circuits.append(p1_in.union(p2_in))

    return reduce(mul, sorted(map(len, circuits), reverse=True)[:3], 1)


def part2(data: List[str]) -> Any:
    """ 2025 Day 8 Part 2
    """
    points = set(map(lambda l: tuple(map(int, re.findall(r'\d+', l))), data))
    distances = [(sum(map(lambda cs: (cs[0] - cs[1]) ** 2, zip(p1, p2))), p1, p2) for p1, p2 in combinations(points, 2) if p1 != p2]
    heapq.heapify(distances)

    circuits: List[set[Tuple[int,]]] = []
    while distances:
        dist, p1, p2 = heapq.heappop(distances)
        p1_in = next(map(lambda it: circuits.pop(it[0]), filter(lambda it: p1 in it[1], enumerate(circuits))), {p1})
        p2_in = next(map(lambda it: circuits.pop(it[0]), filter(lambda it: p2 in it[1], enumerate(circuits))), {p2})
        if points.issubset(p1_in.union(p2_in)):
            return p1[0] * p2[0]
        circuits.append(p1_in.union(p2_in))

    raise ValueError("No solution found")


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
        print(f"\nPart 1:\nProduct of 3 largest circuit sizes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nProduct of x-coordinates of connection that fully connects: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
