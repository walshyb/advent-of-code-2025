grid = []

with open("input.txt", "r") as file:
    for line in file:
        grid.append(list(line.strip()))


def is_valid(current_row, current_col, row_offset, col_offset):
    return (
        current_row + row_offset >= 0
        and current_row + row_offset < len(grid)
        and current_col + col_offset >= 0
        and current_col + col_offset < len(grid[0])
    )


root = (0, 0)
stack = [root]
seen = set()
result = 0

# part 2!
while True:
    current_grid = [row[:] for row in grid]

    # part 1!
    while stack:
        current = stack.pop()
        row, col = current

        if current in seen:
            continue

        seen.add(current)

        nearest_rolls = []

        for i in range(-1, 2):
            for j in range(-1, 2):
                if i == 0 and j == 0 or not is_valid(row, col, i, j):
                    continue

                child = (row + i, col + j)
                stack.append(child)

                if current_grid[child[0]][child[1]] != ".":
                    nearest_rolls.append(child)

        if len(nearest_rolls) < 4:
            if current_grid[row][col] != ".":
                current_grid[row][col] = "."
                result += 1

    if grid == current_grid:
        # print(grid)
        break

    grid = [row[:] for row in current_grid]
    seen.clear()
    stack = [root]


for row in grid:
    for col in row:
        print(col, end="")
    print()
print(result)
