from pathlib import Path
import itertools

file_path = Path(".").resolve().parent / "data" / "day_11.txt"


assert file_path.exists()


content = file_path.read_text().strip()
numbers = list(map(int, content.split()))

print(numbers)


cache = {}


def blink(number: int, depth: int) -> int:

    if depth == 0:
        return 1

    if (number, depth) in cache:
        return cache[(number, depth)]

    string = str(number)

    if number == 0:
        numbers = [1]

    elif len(string) % 2 == 0:
        n = len(string)
        numbers = [int(string[: n // 2]), int(string[n // 2 :])]
    else:
        numbers = [number * 2024]

    res = sum([blink(n, depth - 1) for n in numbers])

    cache[(number, depth)] = res
    return res


print(len(numbers))

print(sum([blink(n, 75) for n in numbers]))
