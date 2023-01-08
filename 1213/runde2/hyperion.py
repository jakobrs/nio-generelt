import itertools
from typing import Tuple, List, Set


w, h = map(int, input().split())

map = [
    input()
    for _ in range(h)
]

start = None
goal = None

for r, line in enumerate(map):
    for c, ch in enumerate(line):
        if ch == 'S':
            start = (r, c)
        elif ch == 'M':
            goal = (r, c)

Node = Tuple[Tuple[int, int], int]

frontier: List[Node] = [(start, 0)]
next: List[Node] = []
visited: Set[Node] = set()

direction_to_diff = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
]

for i in itertools.count():
    if not frontier:
        print("NO")
        exit()

    for node in frontier:
        pos, dir = node
        if pos == goal:
            print(i)
            exit()

        if True:
            r, c = pos
            dr, dc = direction_to_diff[dir]
            r, c = r + dr, c + dc
            if 0 <= r < h and 0 <= c < w:
                new_node = ((r, c), dir)
                if map[r][c] != '#' and new_node not in visited:
                    visited.add(new_node)
                    next.append(new_node)

        for dd in [-1, 1]:
            new_dir = (dir + dd) % 4
            new_node = (pos, new_dir)

            if new_node not in visited:
                visited.add(new_node)
                next.append(new_node)

    frontier = next
    next = []
