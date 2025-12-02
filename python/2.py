import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

from requests import get

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from math import log10, floor, ceil, sqrt


def get_ranges(data: List[str]) -> List[Tuple[int, int]]:
    range_list = []
    for range_pair in next(iter(data)).split(","):
        start, end = map(int, range_pair.split("-"))
        sdig, edig = map(lambda n: ceil(log10(n)), (start, end))
        for digs in range(sdig, edig + 1):
            range_list.append((max(start, 10**(digs - 1)), min(end, 10**digs - 1)))

    return range_list


def set_repeated_digits(start: int, end: int, repeat_count: int) -> set[int]:
    digs = ceil(log10(start + 1))
    if digs % repeat_count != 0:
        return set()
    count = set()
    div = sum(10**i for i in range(0, digs, digs // repeat_count))
    for mul in range(ceil(start / div), floor(end / div) + 1):
        count.add(div * mul)

    return count


def get_divisors(n):
    divisors = []
    for i in range(1, int(sqrt(n)) + 1):
        if n % i == 0:
            divisors.append(i)
            if i * i != n:
                divisors.append(n // i)

    return divisors


def part1(data: List[str]) -> Any:
    """ 2025 Day 2 Part 1
    >>> part1(["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"])
    1227775554
    """
    count = 0
    for range_pair in get_ranges(data):
        count += sum(set_repeated_digits(*range_pair, 2))

    return count


def part2(data: List[str]) -> Any:
    """ 2025 Day 2 Part 2
    >>> part2(["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"])
    4174379265
    """
    count = 0
    for range_pair in next(iter(data)).split(","):
        range_reps = set()
        for s, e in get_ranges([range_pair]):
            digs = ceil(log10(s + 1))
            for repeat_count in filter(lambda d: d > 1, get_divisors(digs)):
                range_reps |= set_repeated_digits(s, e, repeat_count)
        count += sum(range_reps)

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
        print(f"\nPart 1:\nSum of invalid IDs: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of invalid IDs: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
