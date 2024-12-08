example_input = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""


class Coord:
    def __init__(self, x: int, y: int) -> None:
        if x < 0 or y < 0:
            raise ValueError(f"Invalid coords: {x}, {y}")

        self.x = x
        self.y = y

    def to_matrix(self):
        return (self.y, self.x)

    def turn_to(self, direction: tuple[int, int]):
        return Coord(self.x - direction[0], self.y - direction[1])

    def __str__(self) -> str:
        return str((self.x, self.y))


def get_matrix_pos_from(coord: Coord, matrix: list[str]):
    pos = coord.to_matrix()
    return matrix[pos[0]][pos[1]]


def mark_visited(coord: Coord, matrix: list[str]):
    pos = coord.to_matrix()
    split_row = list(matrix[pos[0]])
    split_row[pos[1]] = "X"
    matrix[pos[0]] = "".join(split_row)
    return matrix


def room_to_matrix(string: str):
    lines = string.splitlines()
    return lines, len(lines), len(lines[0])


def get_starting_pos(string: str):
    lines: list[str] = string.splitlines()
    for i, row in enumerate(lines):
        if "^" in row:
            return Coord(row.index("^"), i)

    raise ValueError("Could not find starting pos")


with open("./puzzle_input.txt", "r") as file:
    puzzle_input = file.read().strip()

room_str = puzzle_input
room, rows, cols = room_to_matrix(room_str)

current_pos = get_starting_pos(room_str)
direction = (0, 1)


def get_new_direction(direction: tuple[int, int]) -> tuple[int, int]:
    match direction:
        case (0, 1):
            return (-1, 0)
        case (-1, 0):
            return (0, -1)
        case (0, -1):
            return (1, 0)
        case (1, 0):
            return (0, 1)

    raise ValueError(f"Invalid direction: {direction}")


print(current_pos)
import os

while True:
    try:
        new_pos = current_pos.turn_to(direction)
        object_at_pos = get_matrix_pos_from(new_pos, room)
    except IndexError:
        break

    if object_at_pos == "#":
        direction = get_new_direction(direction)
        just_hit_object = True
    else:
        room = mark_visited(new_pos, room)
        current_pos = new_pos

    print("\n".join(room))
    os.system("clear")

print()
print("Steps taken:", "\n".join(room).count("X"))
