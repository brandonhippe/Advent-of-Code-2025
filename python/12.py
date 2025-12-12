import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

import numpy as np
from scipy.signal import convolve2d
from itertools import chain
from functools import reduce
from operator import mul


def part1(data: List[str]) -> Any:
    """ 2025 Day 12 Part 1
    """
    shapes = list(map(lambda x: x.splitlines(), re.split('\n\n', '\n'.join(data))))
    regions = shapes.pop()

    shapes = list(np.array([[c == '#' for c in line] for line in shape[1:]], dtype=int) for shape in shapes)

    count = 0
    for region_line in regions:
        nums = map(lambda x: int(x.group()), re.finditer(r'\d+', region_line))
        shape, counts = (next(nums), next(nums)), list(nums)

        if sum(map(lambda ix_c: reduce(mul, shapes[ix_c[0]].shape, 1) * ix_c[1], enumerate(counts))) <= shape[0] * shape[1]:
            count += 1
            continue

        if sum(sum(map(lambda ix_c: sum(shapes[ix_c[0]]) * ix_c[1], enumerate(counts)))) > shape[0] * shape[1]:
            continue

        # This turns out to not be necessary for my input, but I'm leaving it here in case it's needed for other inputs
        test_count = 1
        filled_regions = [np.zeros((shape[0], shape[1]), dtype=int)]
        for shape_ix in chain.from_iterable(map(lambda x: (x[0],) * x[1], enumerate(counts))):
            shape = shapes[shape_ix]
            new_filled = set()

            for rem_count in range(test_count):
                for region in filled_regions:
                    for loc in zip(*np.where(convolve2d(region, np.fliplr(np.flipud(shape)), mode='same', boundary='fill', fillvalue=1) == 0)):
                        new_region = region + np.pad(shape, pad_width=((loc[0] - shape.shape[0] // 2, region.shape[0] - (shape.shape[0] + loc[0] - shape.shape[0] // 2)), (loc[1] - shape.shape[1] // 2, region.shape[1] - (shape.shape[1] + loc[1] - shape.shape[1] // 2))), mode='constant', constant_values=0)
                        existing = False
                        for check_count in range(8):
                            if tuple(new_region.flatten()) in new_filled:
                                existing = True
                                break
                            if check_count % 2:
                                shape = np.rot90(shape)
                            else:
                                shape = np.fliplr(shape)
                                
                        if not existing:
                            new_filled.add(tuple(new_region.flatten()))
            
                if rem_count % 2:
                    shape = np.rot90(shape)
                else:
                    shape = np.fliplr(shape)

            test_count = 8
            filled_regions = list(map(lambda x: np.array(list(x)).reshape(*filled_regions[-1].shape), new_filled))
            if not filled_regions:
                break
        
        count += bool(filled_regions)
    return count


def part2(data: List[str]) -> Any:
    """ 2025 Day 12 Part 2
    """
    return "Christmas has been saved!"


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
        print(f"\nPart 1:\nRegions that can fit the listed presents: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
