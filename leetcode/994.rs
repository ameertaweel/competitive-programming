// LeetCode/994 - Rotting Oranges

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut grid_curr = grid;
        let mut grid_next = grid_curr.clone();

        let mut minute = 0;
        let mut changed = 0;
        let mut fresh = 0;

        loop {
            for r in 0..rows {
                for c in 0..cols {
                    grid_next[r][c] = grid_curr[r][c];
                    if grid_curr[r][c] != 1 {
                        continue;
                    }
                    fresh += 1;
                    if (r > 0 && grid_curr[r - 1][c] == 2)
                        || (r < rows - 1 && grid_curr[r + 1][c] == 2)
                        || (c > 0 && grid_curr[r][c - 1] == 2)
                        || (c < cols - 1 && grid_curr[r][c + 1] == 2)
                    {
                        changed += 1;
                        grid_next[r][c] = 2;
                        continue;
                    }
                }
            }

            if changed == 0 {
                return if fresh > 0 { -1 } else { minute };
            }
            minute += 1;
            changed = 0;
            fresh = 0;
            (grid_curr, grid_next) = (grid_next, grid_curr);
        }
    }
}
