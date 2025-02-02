impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut i = 0;
        let mut j = rows * cols;

        while i < j {
            let mid = i + (j - i) / 2;

            let n = matrix[mid / cols][mid % cols];

            if n == target {
                return true;
            }

            if n < target {
                i = mid + 1;
                continue;
            }

            if n > target {
                j = mid;
                continue;
            }
        }

        return false;
    }
}
