// LeetCode/51 - N-Queens

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut brd = vec![vec![b'.'; n]; n];
        let mut ans = vec![];

        for col in 0..n {
            Self::solve_n_queens_helper(n, &mut brd, &mut ans, 0, 0, 0, 0, col);
        }

        return ans;
    }

    pub fn solve_n_queens_helper(
        n: usize,
        brd: &mut Vec<Vec<u8>>,
        ans: &mut Vec<Vec<String>>,
        cols: usize,
        diags1: usize,
        diags2: usize,
        row: usize,
        col: usize,
    ) {
        if (cols & (1 << col) != 0) {
            return;
        }
        let diag1 = col - row + (n - 1);
        if (diags1 & (1 << diag1) != 0) {
            return;
        }
        let diag2 = (n - col - 1) - row + (n - 1);
        if (diags2 & (1 << diag2) != 0) {
            return;
        }

        brd[row][col] = b'Q';

        let cols = cols | (1 << col);
        let diags1 = diags1 | (1 << diag1);
        let diags2 = diags2 | (1 << diag2);

        if (row == n - 1) {
            ans.push(
                brd.into_iter()
                    .map(|b| unsafe { String::from_utf8_unchecked(b.clone()) })
                    .collect(),
            );
        } else {
            for col in 0..n {
                Self::solve_n_queens_helper(n, brd, ans, cols, diags1, diags2, row + 1, col);
            }
        }

        brd[row][col] = b'.';
    }
}
