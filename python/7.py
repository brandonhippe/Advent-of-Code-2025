import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

from collections import deque, defaultdict
from functools import cache


def part1(data: List[str]) -> Any:
    """ 2025 Day 7 Part 1
    """
    start = None
    splitters = set()
    for y, line in enumerate(filter(lambda l: set(l) != {"."}, data)):
        for x, c in enumerate(line):
            if c == 'S':
                start = (x, y)
            elif c == '^':
                splitters.add((x, y))
        max_y = y

    assert start is not None, "Did not find the starting position"

    open_list = deque([start])
    finished = set()
    splits = 0
    while open_list:
        x, y = open_list.popleft()

        if (x, y) in finished:
            continue
        finished.add((x, y))

        nx, ny = x, y + 1
        if ny > max_y or nx < 0 or nx > len(data[0]):
            # Cannot split any further, stop checking
            continue
        elif (nx, ny) in splitters:
            nx = (nx - 1, nx + 1)
            splits += 1
        else:
            nx = (nx,)

        for n_x in nx:
            if (n_x, ny) not in finished:
                open_list.append((n_x, ny))

    return splits


def part2(data: List[str]) -> Any:
    """ 2025 Day 7 Part 2
    """
    start = None
    splitters = set()
    for y, line in enumerate(filter(lambda l: set(l) != {"."}, data)):
        for x, c in enumerate(line):
            if c == 'S':
                start = (x, y)
            elif c == '^':
                splitters.add((x, y))
        max_y = y

    assert start is not None, "Did not find the starting position"

    open_list: deque[tuple[tuple[int, int], tuple[int, int]]] = deque([(start, start)])
    finished = set()
    splits = defaultdict(set)
    endpoints = set()
    while open_list:
        pos, path_start = open_list.popleft()
        adding = pos not in finished

        x, y = pos
        nx, ny = x, y + 1
        if ny > max_y or nx < 0 or nx > len(data[0]):
            # Cannot split any further, stop checking
            endpoints.add((nx, ny))
            splits[(nx, ny)].add(path_start)
            continue
        elif (nx, ny) in splitters:
            finished.add(pos)
            splits[(nx, ny)].add(path_start)
            nx = zip((nx - 1, nx + 1), ((nx, ny), (nx, ny)))
        else:
            nx = ((nx, path_start),)

        if adding:
            for n_x, n_p in nx:
                open_list.append(((n_x, ny), n_p))

    @cache
    def calc_timelines(p):
        if p in splits:
            return sum(map(calc_timelines, splits[p]))
        return 1

    return sum(map(calc_timelines, endpoints))


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
        print(f"\nPart 1:\nNumber of splits: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of timelines: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
