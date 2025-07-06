// LeetCode/73 - Set Matrix Zeroes

use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (rows, cols) = (matrix.len(), matrix[0].len());

        let mut rows_to_zero = HashSet::new();
        let mut cols_to_zero = HashSet::new();

        for row in 0..rows {
            for col in 0..cols {
                if matrix[row][col] == 0 {
                    rows_to_zero.insert(row);
                    cols_to_zero.insert(col);
                }
            }
        }

        for &row in &rows_to_zero {
            for col in 0..cols {
                matrix[row][col] = 0;
            }
        }
        for &col in &cols_to_zero {
            for row in 0..rows {
                matrix[row][col] = 0;
            }
        }
    }
}
