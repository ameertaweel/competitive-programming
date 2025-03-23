// LeetCode/417 - Pacific Atlantic Water Flow

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = heights.len();
        let cols = heights[0].len();

        if rows == 1 && cols == 1 {
            return vec![vec![0, 0]];
        }

        let mut vals_pacific = vec![vec![false; cols]; rows];
        let mut vals_atlantic = vec![vec![false; cols]; rows];

        let mut stack_pacific = vec![];
        let mut stack_atlantic = vec![];

        for row in 0..rows {
            vals_pacific[row][0] = true;
            vals_atlantic[row][cols - 1] = true;

            stack_pacific.push((row, 0));
            stack_atlantic.push((row, cols - 1));
        }
        for col in 0..cols {
            vals_pacific[0][col] = true;
            vals_atlantic[rows - 1][col] = true;

            stack_pacific.push((0, col));
            stack_atlantic.push((rows - 1, col));
        }

        Self::dfs(&heights, &mut vals_pacific, &mut stack_pacific);
        Self::dfs(&heights, &mut vals_atlantic, &mut stack_atlantic);

        let mut ans = vec![];

        for row in 0..rows {
            for col in 0..cols {
                if vals_pacific[row][col] && vals_atlantic[row][col] {
                    ans.push(vec![row as i32, col as i32]);
                }
            }
        }

        return ans;
    }

    pub fn dfs(
        heights: &Vec<Vec<i32>>,
        vals: &mut Vec<Vec<bool>>,
        stack: &mut Vec<(usize, usize)>,
    ) {
        let rows = heights.len();
        let cols = heights[0].len();

        while stack.len() > 0 {
            let (row, col) = stack.pop().unwrap();
            let height = heights[row][col];
            for (delta_row, delta_col) in DIRS {
                let row_next = row as i32 + delta_row;
                if row_next < 0 || row_next >= rows as i32 {
                    continue;
                }
                let col_next = col as i32 + delta_col;
                if col_next < 0 || col_next >= cols as i32 {
                    continue;
                }
                let (row_next, col_next) = (row_next as usize, col_next as usize);
                if heights[row_next][col_next] < height {
                    continue;
                }
                if vals[row_next][col_next] {
                    continue;
                }

                vals[row_next][col_next] = true;
                stack.push((row_next, col_next));
            }
        }
    }
}
