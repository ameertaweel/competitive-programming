# LeetCode/286 - Walls and Gates


class Solution:
    def islandsAndTreasure(self, grid: list[list[int]]) -> None:
        rows = len(grid)
        cols = len(grid[0])

        DIRS = ((-1, 0), (1, 0), (0, -1), (0, 1))

        for r in range(rows):
            for c in range(cols):
                if grid[r][c] != 0:
                    continue
                queue = [(r, c)]
                while len(queue) > 0:
                    r, c = queue.pop(0)
                    d = grid[r][c]
                    for delta_r, delta_c in DIRS:
                        r_neigh = r + delta_r
                        if r_neigh < 0 or r_neigh >= rows:
                            continue
                        c_neigh = c + delta_c
                        if c_neigh < 0 or c_neigh >= cols:
                            continue
                        if d + 1 < grid[r_neigh][c_neigh]:
                            grid[r_neigh][c_neigh] = d + 1
                            queue.append((r_neigh, c_neigh))
