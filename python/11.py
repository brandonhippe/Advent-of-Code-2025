import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer

from collections import defaultdict, deque
import heapq
from functools import cache, reduce


@cache
def count_paths(connections: Tuple[int], start: int, end: int, ignore: int) -> int:
    if start == end:
        return 1

    paths = 0
    connection_val = connections[start]
    ix = 0
    while connection_val:
        if connection_val & 1 and not (ignore & (1 << ix)):
            paths += count_paths(connections, ix, end, ignore | (1 << ix))
        connection_val >>= 1
        ix += 1

    return paths


def part1(data: List[str]) -> Any:
    """ 2025 Day 11 Part 1
    """
    word_re = re.compile(r'\w+')
    connections = {}
    all_words = set()
    for line in data:
        words = word_re.finditer(line)
        f = next(words).group(0)
        connections[f] = set(map(lambda m: m.group(0), words))

        all_words.add(f)
        all_words |= connections[f]

    word_to_ix = {word: ix for ix, word in enumerate(sorted(all_words))}
    connections = tuple(
        reduce(
            lambda acc, w: acc | (1 << word_to_ix[w]),
            connections.get(word, set()),
            0
        )
        for word in sorted(all_words)
    )

    return count_paths(connections, word_to_ix["you"], word_to_ix["out"], 1 << word_to_ix["you"])


def part2(data: List[str]) -> Any:
    """ 2025 Day 11 Part 2
    """
    word_re = re.compile(r'\w+')
    connections = {}
    total_connections = defaultdict(int)
    all_words = set()
    for line in data:
        words = word_re.finditer(line)
        f = next(words).group(0)
        connections[f] = set(map(lambda m: m.group(0), words))

        all_words.add(f)
        all_words |= connections[f]

        # total_connections[f] += len(connections[f])
        for t in connections[f]:
            total_connections[t] += 1

    # Determine important nodes (most connected)
    total_connections = list((-cnt, w) for w, cnt in total_connections.items())
    heapq.heapify(total_connections)
    max_connections, node = heapq.heappop(total_connections)

    important_nodes = {node, "svr", "dac", "fft", "out"}
    while total_connections:
        cnt, node = heapq.heappop(total_connections)
        if -cnt < -max_connections // 2:
            break
        important_nodes.add(node)
    
    # Create 2nd representation with only important nodes
    open_list = deque(list(('svr', k) for k in connections['svr']))
    visited = set()
    important_connections = defaultdict(set)
    while open_list:
        f, t = open_list.popleft()
        if (f, t) in visited:
            continue
        visited.add((f, t))

        if t in important_nodes:
            important_connections[f].add(t)
            f = t

        for k in filter(lambda k: (f, k) not in visited, connections.get(t, set())):
            open_list.append((f, k))

    # If a node connects to dac or fft, only keep those connections
    for k, v in important_connections.items():
        if (new_v := v & {"dac", "fft"}):
            important_connections[k] = new_v

    # Find all paths from svr to out using important connections
    main_paths = set()
    open_list = deque([('svr',)])
    while open_list:
        path = open_list.popleft()
        last_node = path[-1]

        if last_node == "out":
            main_paths.add(path)
            continue

        for conn in important_connections.get(last_node, set()):
            if conn not in path:
                open_list.append(path + (conn,))

    main_paths = set(filter(lambda p: "dac" in p and "fft" in p, main_paths))

    # Convert paths to bitmask representation
    all_words = sorted(all_words)
    word_to_ix = {word: ix for ix, word in enumerate(all_words)}
    important_num = sum(1 << word_to_ix[w] for w in important_nodes)

    connections = tuple(
        reduce(
            lambda acc, w: acc | (1 << word_to_ix[w]),
            connections.get(word, set()),
            0
        )
        for word in all_words
    )

    total = 0
    for path in main_paths:
        path_total = 1
        for i in range(len(path) - 1):
            path_total *= count_paths(
                connections,
                word_to_ix[path[i]],
                word_to_ix[path[i + 1]],
                important_num ^ (1 << word_to_ix[path[i + 1]])
            )
        total += path_total

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
        print(f"\nPart 1:\nNumber of paths from 'you' to 'out': {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of paths from 'svr' to 'out' through both 'dac' and 'fft': {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
