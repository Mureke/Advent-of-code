import os

from collections import Counter


def solve():
    with open(os.path.abspath('data/day_5.txt')) as file:
        data = file.read().replace(' -> ', ',').split('\n')

    straight_lines = []
    diagonal_lines = []
    for line in data:
        x1, y1, x2, y2 = map(int, line.split(','))
        (x1, y1), (x2, y2) = sorted([(x1, y1), (x2, y2)])

        if x1 == x2 or y1 == y2:
            straight_lines += [(x, y) for x in range(x1, x2 + 1) for y in range(y1, y2 + 1)]

        elif y1 < y2:
            for i, x in enumerate(range(x1, x2+1)):
                diagonal_lines.append((x, y1+i))
        else:
            for i, x in enumerate(range(x1, x2+1)):
                diagonal_lines.append((x, y1-i))
    counter = Counter(straight_lines)
    res = list(filter(None, [v > 1 for v in counter.values()]))
    print(f"Day 5 Part1: {len(res)}")

    counter += Counter(diagonal_lines)

    res2 = list(filter(None, [v > 1 for v in counter.values()]))
    print(f"Day 5 Part2: {len(res2)}")
