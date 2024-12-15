from dataclasses import dataclass
from pathlib import Path
import itertools
from time import sleep
import numpy as np

file_path = Path(".").resolve().parent / "data" / "day_15.txt"

assert file_path.exists(), file_path


content = file_path.read_text().strip()
map, moves = content.split("\n\n")


class MapObject:
    def __init__(self, i, j):
        self.i = i
        self.j = j


class Block:
    pass


class Robot:
    pass


class Box:
    pass


def parse_map(map):
    lines = map.split("\n")
    chars = [[c for c in line] for line in lines]
    return chars
    map = []
    for i, line in enumerate(map.split("\n")):
        line_map = []
        for j, char in enumerate(line):
            match char:
                case "#":
                    line_map.append(Block(i, j))
                case _:
                    pass


def parse_moves(moves):
    return "".join(moves.split("\n"))


map = parse_map(map)
print(map)


moves = parse_moves(moves)

width, height = len(map[0]), len(map)

# Find robot
robot = None
for i in range(height):
    for j in range(width):
        if map[i][j] == "@":
            robot = (i, j)


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


def set_at(map, pos, val):
    map[pos[0]][pos[1]] = val


def apply_move(source, direction, map):
    target_pos = (source[0] + direction[0], source[1] + direction[1])

    match map[source[0]][source[1]]:
        case "#":
            return False
        case ".":
            return True
        case "O" | "@":
            if not apply_move(target_pos, direction, map):
                return False
            set_at(map, target_pos, map[source[0]][source[1]])
            set_at(map, source, ".")
            return True


def print_map(map):
    print("\n".join(["".join(line) for line in map]))


print_map(map)

# Perform robot moves
for move in moves:

    dir = translate_direction(move)
    if apply_move(robot, dir, map):
        robot = (robot[0] + dir[0], robot[1] + dir[1])

print(robot)
print_map(map)


def compute_score(map):
    score = 0
    for i in range(height):
        for j in range(width):
            if map[i][j] == "O":
                score += i * 100 + j
    return score


print(compute_score(map))
