use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_sums = vec![vec![false; 9]; 9];
        let mut col_sums = vec![vec![false; 9]; 9];
        let mut sub_sums = vec![vec![false; 9]; 9];

        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    continue;
                }

                let n = board[row][col].to_digit(10).unwrap() as usize;
                let sub = (row / 3) * 3 + (col / 3);

                if row_sums[row][n - 1] == true {
                    return false;
                }
                if col_sums[col][n - 1] == true {
                    return false;
                }
                if sub_sums[sub][n - 1] == true {
                    return false;
                }

                row_sums[row][n - 1] = true;
                col_sums[col][n - 1] = true;
                sub_sums[sub][n - 1] = true;
            }
        }

        return true;
    }

    pub fn is_valid_sudoku_alt(board: Vec<Vec<char>>) -> bool {
        let mut row_sums = vec![HashSet::new(); 9];
        let mut col_sums = vec![HashSet::new(); 9];
        let mut sub_sums = vec![HashSet::new(); 9];

        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    continue;
                }

                let n = board[row][col].to_digit(10).unwrap();
                let sub = (row / 3) * 3 + (col / 3);

                if row_sums[row].contains(&n) {
                    return false;
                }
                if col_sums[col].contains(&n) {
                    return false;
                }
                if sub_sums[sub].contains(&n) {
                    return false;
                }

                row_sums[row].insert(n);
                col_sums[col].insert(n);
                sub_sums[sub].insert(n);
            }
        }

        return true;
    }
}
