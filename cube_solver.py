WIDTH = 5
DEPTH = 5
HEIGHT = 5

Cube = [[[0 for k in range(WIDTH)] for j in range(DEPTH)] for i in range(HEIGHT)]

Configurations = [
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [1, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [2, 1, 0]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [1, 0, 1]],
    [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [2, 0, 1]],
    [[0, 0, 0], [-2, 1, 0], [-1, 1, 0], [0, 1, 0], [1, 1, 0]],
    [[0, 0, 0], [-1, 1, 0], [0, 1, 0], [1, 1, 0], [2, 1, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [-1, 1, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [0, 1, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [1, 1, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [-1, 2, 0]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [0, 2, 1]],
    [[0, 0, 0], [0, 1, 0], [0, 2, 0], [0, 3, 0], [1, 2, 0]],
    [[0, 0, 0], [0, 0, 1], [-2, 0, 1], [-1, 0, 1], [1, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [-1, 0, 1], [1, 0, 1], [2, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [0, -2, 1], [0, -1, 1], [0, 1, 1]],
    [[0, 0, 0], [0, 0, 1], [0, -1, 1], [0, 1, 1], [0, 2, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [-1, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, -1, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [1, 0, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 1, 1]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [-1, 0, 2]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, -1, 2]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [1, 0, 2]],
    [[0, 0, 0], [0, 0, 1], [0, 0, 2], [0, 0, 3], [0, 1, 2]]
]


def print_cube():
    for level in Cube:
        for row in level:
            print(row)
        print()
    print("-" * 15)


def remove_piece(n):
    for z in range(0, HEIGHT):
        for y in range(0, DEPTH):
            for x in range(0, WIDTH):
                if Cube[z][y][x] == n:
                    Cube[z][y][x] = 0


def insert_piece(x, y, z, c, n):
    config = Configurations[c]

    # Check if the piece fits
    for b in config:
        x_b = x + b[0]
        y_b = y + b[1]
        z_b = z + b[2]
        # Out of bounds
        if (x_b < 0) or (x_b >= WIDTH) or (y_b < 0) or (y_b >= DEPTH) or (z_b < 0) or (z_b >= HEIGHT):
            return False
        # Slot taken
        if Cube[z_b][y_b][x_b] != 0:
            return False

    # Insert piece
    for b in config:
        Cube[z+b[2]][y+b[1]][x+b[0]] = n
    return True


def find_pos():
    for z in range(0, HEIGHT):
        for y in range(0, DEPTH):
            for x in range(0, WIDTH):
                if Cube[z][y][x] == 0:
                    return[x, y, z]


def find_solution(n):

    if n > 25:
        print_cube()
        return

    [x, y, z] = find_pos()

    for c in range(len(Configurations)):
        if insert_piece(x, y, z, c, n):
            find_solution(n+1)
            remove_piece(n)


find_solution(1)
