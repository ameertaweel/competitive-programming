// LeetCode/48 - Rotate Image

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        if n == 1 {
            return;
        }

        let mut row_1 = 0;
        let mut col_1 = 0;
        let mut val = matrix[row_1][col_1];

        let mut rep = 0;
        let mut reps = n - 1;
        let mut side = 0;
        let mut i = 0;

        loop {
            let row_2 = col_1;
            let col_2 = n - row_1 - 1;
            let val_next = matrix[row_2][col_2];

            matrix[row_2][col_2] = val;

            (row_1, col_1) = (row_2, col_2);
            val = val_next;

            side += 1;
            if side < 4 {
                continue;
            }

            side = 0;
            col_1 += 1;
            val = matrix[row_1][col_1];
            rep += 1;
            if rep < reps {
                continue;
            }

            if reps <= 2 {
                break;
            }

            i += 1;
            rep = 0;
            row_1 = i;
            col_1 = i;
            reps -= 2;
            val = matrix[row_1][col_1];
        }
    }
}
