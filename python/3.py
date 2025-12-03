import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

def biggest_ix_num(line: str, rem: int) -> Tuple[int, int]:
    s = slice(-(rem-1)) if rem > 1 else slice(None)
    return max(enumerate(map(int, line[s])), key=lambda e: e[1])


def part1(data: List[str]) -> Any:
    """ 2025 Day 3 Part 1
    >>> part1(['987654321111111', '811111111111119', '234234234234278', '818181911112111'])
    357
    """
    count = 0
    for line in data:
        ix, tens = biggest_ix_num(line, 2)
        ones = max(map(int, line[ix+1:]))
        count += 10 * tens + ones

    return count


def part2(data: List[str]) -> Any:
    """ 2025 Day 3 Part 2
    >>> part2(['987654321111111', '811111111111119', '234234234234278', '818181911112111'])
    3121910778619
    """
    count = 0
    for line in data:
        num = 0
        for rem in range(12,0,-1):
            ix, digit = biggest_ix_num(line, rem)
            num = 10 * num + digit
            line = line[ix+1:]
        count += num

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
        print(f"\nPart 1:\nJoltage: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nJoltage: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
