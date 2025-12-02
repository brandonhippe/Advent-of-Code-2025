import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from math import ceil, floor


def part1(data: List[str]) -> Any:
    """ 2025 Day 1 Part 1
    >>> part1(["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"])
    3
    """
    dial = 50
    count = 0
    for line in data:
        mult = -1 if line[0] == 'L' else 1
        dial = (dial + mult * int(line[1:])) % 100
        count += dial == 0

    return count


def part2(data: List[str]) -> Any:
    """ 2025 Day 1 Part 2
    >>> part2(["L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82"])
    6
    """
    dial = 50
    count = 0
    for line in data:
        mult = -1 if line[0] == 'L' else 1
        amt = int(line[1:])
        while amt > 0:
            dial += mult
            dial %= 100
            amt -= 1
            count += dial == 0

    return count


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
        print(f"\nPart 1:\nPassword: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPassword: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
