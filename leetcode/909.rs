use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let targetLabel = n * n;

        let mut q = VecDeque::new();
        let mut seen = vec![false; n * n];

        q.push_back((1, 0));
        seen[0] = true;

        while let Some((label, rolls)) = q.pop_front() {
            if label == targetLabel {
                return rolls;
            }

            let (row, col) = labelToCell(n, label);

            for next in (label + 1)..=std::cmp::min(label + 6, targetLabel) {
                let (row, col) = labelToCell(n, next);
                let next = if board[row][col] == -1 {
                    next
                } else {
                    board[row][col] as usize
                };
                if seen[next - 1] {
                    continue;
                }
                q.push_back((next, rolls + 1));
                seen[next - 1] = true;
            }
        }

        return -1;
    }
}

fn labelToCell(n: usize, label: usize) -> (usize, usize) {
    let label = label - 1; // Start counting from 0

    let row = label / n;
    let col = label % n;

    let col = if row % 2 == 0 {
        col
    } else {
        n - 1 - col // Columns alternate direction
    };
    let row = n - 1 - row; // Last row has label 0

    return (row, col);
}
