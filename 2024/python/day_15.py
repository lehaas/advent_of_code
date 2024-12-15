from dataclasses import dataclass
from pathlib import Path
import itertools
from time import sleep
import numpy as np

file_path = Path(".").resolve().parent / "data" / "day_15.txt"

assert file_path.exists(), file_path


content = file_path.read_text().strip()
map, moves = content.split("\n\n")


class Free:
    def can_move(self, map, direction):
        return True

    def move(self, map, direction):
        pass

    def marker(self):
        return "."

    def score(self):
        return 0


class Block:
    def __init__(self, i, j):
        self.i = i
        self.j = j

    def marker(self):
        return "#"

    def can_move(self, map, direction):
        return False

    def move(self, map, direction):
        assert False, "Block cannot be moved."

    def score(self):
        return 0


class Robot:
    def __init__(self, i, j):
        self.i = i
        self.j = j

    def marker(self):
        return "@"

    def move(self, map, direction: tuple[int, int]):
        target = map[self.i + direction[0]][self.j + direction[1]]
        if target.can_move(map, direction):
            target.move(map, direction)
            map[self.i + direction[0]][self.j + direction[1]] = self
            map[self.i][self.j] = FREE
            self.i += direction[0]
            self.j += direction[1]

    def can_move(self, map, direction):
        assert False, "Robot should never be moved"

    def score(self):
        return 0


class Box:
    def __init__(self, i, j):
        self.i = i
        self.j = j
        size = (1, 2)

    def marker(self):
        return "O"

    def _get_target_locations(self, direction):
        match direction:
            case (0, 1):  # move right
                return [(self.i, self.j + 1), (self.i, self.j + 2)]

            case (0, -1):  # move left
                return [(self.i, self.j - 1), (self.i, self.j)]

            case (1, 0):  # move down
                return [(self.i + 1, self.j), (self.i + 1, self.j + 1)]
            case (-1, 0):  # move up
                return [(self.i - 1, self.j), (self.i - 1, self.j + 1)]

    def move(self, map, direction: tuple[int, int]):
        assert self.can_move(map, direction)

        map[self.i][self.j] = FREE
        map[self.i][self.j + 1] = FREE

        for loc in self._get_target_locations(direction):
            if map[loc[0]][loc[1]] is not self:
                map[loc[0]][loc[1]].move(map, direction)
            map[loc[0]][loc[1]] = self

        self.i += direction[0]
        self.j += direction[1]

    def can_move(self, map, direction):
        for loc in self._get_target_locations(direction):
            target = map[loc[0]][loc[1]]
            if target is self:
                continue
            if not map[loc[0]][loc[1]].can_move(map, direction):
                return False
        return True

    def score(self):
        return self.i * 100 + self.j


FREE = Free()


def parse_map(map):
    res = []
    for i, line in enumerate(map.split("\n")):
        line_map = []
        for j, char in enumerate(line):
            match char:
                case "#":
                    obj = Block(i, 2 * j)
                    line_map.extend([obj, obj])
                case ".":
                    line_map.extend([FREE, FREE])
                case "O":
                    obj = Box(i, 2 * j)
                    line_map.extend([obj, obj])
                case "@":
                    obj = Robot(i, 2 * j)
                    line_map.extend([obj, FREE])
        res.append(line_map)
    return res


def print_map(map):
    print("\n".join(["".join(c.marker() for c in line) for line in map]))


map = parse_map(map)
print_map(map)


def parse_moves(moves):
    return "".join(moves.split("\n"))


moves = parse_moves(moves)


def find_robot(map):
    width, height = len(map[0]), len(map)

    for i in range(height):
        for j in range(width):
            if isinstance(map[i][j], Robot):
                return map[i][j]


robot = find_robot(map)

print(robot.i, robot.j)


def translate_direction(direction):
    match direction:
        case "^":
            return (-1, 0)
        case "<":
            return (0, -1)
        case ">":
            return (0, 1)
        case "v":
            return (1, 0)


for i, move in enumerate(moves):
    robot.move(map, translate_direction(move))

print_map(map)


def compute_score(map):

    score = 0
    for line in map:
        for obj in line:
            score += obj.score()
    return score // 2


print(compute_score(map))
