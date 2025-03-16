// LeetCode/695 - Max Area of Island

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut max_size = 0;
        let mut seen = vec![vec![false; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 || seen[r][c] {
                    continue;
                }

                let mut stack = vec![(r, c)];
                seen[r][c] = true;
                let mut size = 0;
                while stack.len() > 0 {
                    let (r, c) = stack.pop().unwrap();
                    size += 1;

                    for (delta_row, delta_col) in DIRECTIONS {
                        let r_next = r as i32 + delta_row;
                        if r_next < 0 || r_next >= rows as i32 {
                            continue;
                        }
                        let c_next = c as i32 + delta_col;
                        if c_next < 0 || c_next >= cols as i32 {
                            continue;
                        }

                        let (r_next, c_next) = (r_next as usize, c_next as usize);
                        if grid[r_next][c_next] == 0 {
                            continue;
                        }
                        if seen[r_next][c_next] {
                            continue;
                        }
                        seen[r_next][c_next] = true;
                        stack.push((r_next, c_next));
                    }
                }
                if size > max_size {
                    max_size = size;
                }
            }
        }

        return max_size;
    }
}
