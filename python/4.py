import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer


from itertools import chain, product
from collections import defaultdict


def part1(data: List[str]) -> Any:
    """ 2025 Day 4 Part 1
    >>> part1(["..@@.@@@@.", "@@@.@.@.@@", "@@@@@.@.@@", "@.@@@@..@.", "@@.@@@@.@@", ".@@@@@@@.@", ".@.@.@.@@@", "@.@@@.@@@@", ".@@@@@@@@.", "@.@.@@@.@."])
    13
    """
    rolls = set(chain.from_iterable(map(lambda y: map(lambda x: (x[0], y[0]), filter(lambda c: c[1] == '@', enumerate(y[1]))), enumerate(data))))
    neighbors = defaultdict(int)
    for x, y in rolls:
        for dx, dy in product((-1, 0, 1), repeat=2):
            if dx == 0 and dy == 0:
                continue
            neighbors[(x + dx, y + dy)] += 1

    return len(set(filter(lambda pos: neighbors[pos] < 4, rolls)))


def part2(data: List[str]) -> Any:
    """ 2025 Day 4 Part 2
    """
    rolls = set(chain.from_iterable(map(lambda y: map(lambda x: (x[0], y[0]), filter(lambda c: c[1] == '@', enumerate(y[1]))), enumerate(data))))
    neighbors = defaultdict(int)
    for x, y in rolls:
        for dx, dy in product((-1, 0, 1), repeat=2):
            if dx == 0 and dy == 0:
                continue
            neighbors[(x + dx, y + dy)] += 1

    total = 0
    while (to_rem := set(filter(lambda pos: neighbors[pos] < 4, rolls))):
        total += len(to_rem)
        for pos in to_rem:
            rolls.remove(pos)
            for dx, dy in product((-1, 0, 1), repeat=2):
                if dx == 0 and dy == 0:
                    continue
                neighbors[(pos[0] + dx, pos[1] + dy)] -= 1
    
    return total


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
        print(f"\nPart 1:\nNumber of accessible rolls: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of accessible rolls: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
