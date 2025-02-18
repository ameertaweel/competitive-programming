use std::collections::HashMap;

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board.clone();
        let mut word: Vec<_> = word.chars().collect();
        let mut freqs = HashMap::new();
        let rows = board.len();
        let cols = board[0].len();
        for row in 0..rows {
            for col in 0..cols {
                *freqs.entry(board[row][col]).or_insert(0) += 1;
            }
        }
        for &c in &word {
            *freqs.entry(c).or_insert(0);
        }

        if freqs[&word[0]] > freqs[&word[word.len() - 1]] {
            word.reverse();
        }

        return Self::exists_helepr_root(&mut board, &word);
    }

    pub fn exist_alt(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board.clone();
        let word: Vec<_> = word.chars().collect();
        return Self::exists_helepr_root(&mut board, &word);
    }

    pub fn exists_helepr_root(board: &mut Vec<Vec<char>>, word: &Vec<char>) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        let mut partial = vec![' '; word.len()];
        for row in 0..rows {
            for col in 0..cols {
                if board[row][col] != word[0] {
                    continue;
                }
                partial[0] = board[row][col];
                board[row][col] = ' ';
                let ans = Self::exists_helepr_child(board, word, &mut partial, row, col, 1);
                board[row][col] = partial[0];
                if ans {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn exists_helepr_child(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        partial: &mut Vec<char>,
        row: usize,
        col: usize,
        len: usize,
    ) -> bool {
        if len == word.len() {
            return true;
        }

        let rows = board.len();
        let cols = board[0].len();

        for &(delta_row, delta_col) in &DIRECTIONS {
            let row = row as i64 + delta_row;
            if row < 0 || row >= rows as i64 {
                continue;
            }
            let col = col as i64 + delta_col;
            if col < 0 || col >= cols as i64 {
                continue;
            }

            let row = row as usize;
            let col = col as usize;
            if board[row][col] != word[len] {
                continue;
            }

            partial[len] = board[row][col];
            board[row][col] = ' ';
            let ans = Self::exists_helepr_child(board, word, partial, row, col, len + 1);
            board[row][col] = partial[len];
            if ans {
                return true;
            }
        }

        return false;
    }
}
