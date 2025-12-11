import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

from collections import deque
from functools import reduce

import z3


def fewest_presses(goal_state: int, buttons: List[int]) -> int:
    visited = {0}
    open_list = deque(list(map(lambda b: (1, b), buttons)))  # (steps, state)
    while open_list:
        steps, state = open_list.popleft()
        if state == goal_state:
            return steps
        
        for new_state in filter(lambda s: s not in visited, map(lambda b: state ^ b, buttons)):
            visited.add(new_state)
            open_list.append((steps + 1, new_state))
    
    raise ValueError("No solution found")


def minimize_button_presses(state: List[int], buttons: List[set[int]]) -> int:
    opt = z3.Optimize()
    coeffs = [z3.Int(f'b{i}') for i in range(len(buttons))]
    for c in coeffs:
        opt.add(c >= 0)

    for counter, count in enumerate(state):
        linear_comb = sum(map(lambda c_b: c_b[0], filter(lambda c_b: counter in c_b[1], zip(coeffs, buttons))))
        opt.add(linear_comb == count)

    opt.minimize(sum(coeffs))
    if opt.check() == z3.sat:
        model = opt.model()
        return sum(model[c].as_long() for c in coeffs)
    
    raise ValueError("No solution found")


def part1(data: List[str]) -> Any:
    """ 2025 Day 10 Part 1
    """
    state_re = re.compile(r"\[([\.#]+)\]")
    button_re = re.compile(r"\(([\d,]+)\)")

    return sum(
        fewest_presses(
            reduce(lambda acc, ix: acc | (1 << ix), map(lambda ix_c: ix_c[0], filter(lambda ix_c: ix_c[1] == '#', enumerate(state_re.search(line).group(1)))), 0),
            list(map(lambda m: reduce(lambda acc, ex: acc | (1 << ex), map(int, m.group(1).split(',')), 0), button_re.finditer(line)))
        ) for line in data
    )


def part2(data: List[str]) -> Any:
    """ 2025 Day 10 Part 2
    """
    button_re = re.compile(r"\(([\d,]+)\)")
    counter_re = re.compile(r"\{([\d,]+)\}")

    return sum(
        map(
            lambda line: minimize_button_presses(
                list(map(int, counter_re.search(line).group(1).split(','))),
                list(map(lambda m: set(map(int, m.group(1).split(','))), button_re.finditer(line)))
            ), 
            data
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
        print(f"\nPart 1:\nFewest number of button presses to set the lighting states: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
