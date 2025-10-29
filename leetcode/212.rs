// LeetCode/5 - Word Search II

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone)]
struct Trie {
    next: Vec<Option<Box<Trie>>>,
    idx: isize,
    words: usize,
}

impl Trie {
    fn new() -> Self {
        return Trie {
            next: vec![Option::None; 26],
            idx: -1,
            words: 0,
        };
    }

    fn insert(&mut self, word: &String, idx: usize) {
        let mut ptr = self;
        for c in word.bytes() {
            let char_code = (c as usize) - ('a' as usize);
            if ptr.next[char_code].is_none() {
                ptr.next[char_code] = Option::Some(Box::new(Self::new()));
            }
            ptr = ptr.next[char_code].as_mut().unwrap();
            ptr.words += 1;
        }
        ptr.idx = idx as isize;
    }

    fn dfs(
        &mut self,
        board: &Vec<Vec<char>>,
        words: &mut Vec<String>,
        row: usize,
        col: usize,
        seen: &mut Vec<Vec<bool>>,
        ans: &mut Vec<String>,
    ) -> usize {
        let mut found = 0;
        let char_code = (board[row][col] as usize) - ('a' as usize);
        if self.next[char_code].is_none() {
            return found;
        }
        let ptr = self.next[char_code].as_mut().unwrap();
        if ptr.idx != -1 {
            let w = std::mem::replace(&mut words[ptr.idx as usize], String::new());
            ans.push(w);
            ptr.idx = -1;
            ptr.words -= 1;
            found = 1;
        }
        if ptr.words == 0 {
            return found;
        }

        let rows = board.len() as isize;
        let cols = board[0].len() as isize;

        for (dr, dc) in &DIRECTIONS {
            let row_next = (row as isize) + dr;
            let col_next = (col as isize) + dc;

            if row_next < 0 || row_next >= rows {
                continue;
            }
            if col_next < 0 || col_next >= cols {
                continue;
            }

            let row_next = row_next as usize;
            let col_next = col_next as usize;

            if seen[row_next][col_next] {
                continue;
            }

            seen[row_next][col_next] = true;
            let found_sub = ptr.dfs(board, words, row_next, col_next, seen, ans);
            found += found_sub;
            ptr.words -= found_sub;
            seen[row_next][col_next] = false;

            if ptr.words == 0 {
                return found;
            }
        }

        return found;
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, mut words: Vec<String>) -> Vec<String> {
        let rows = board.len();
        let cols = board[0].len();

        let mut trie = Trie::new();
        for i in 0..words.len() {
            trie.insert(&words[i], i);
        }

        let mut seen = vec![vec![false; cols]; rows];
        let mut ans = vec![];
        let mut found = 0;
        for row in 0..rows {
            for col in 0..cols {
                seen[row][col] = true;
                found += trie.dfs(&board, &mut words, row, col, &mut seen, &mut ans);
                seen[row][col] = false;
                if found == words.len() {
                    return ans;
                }
            }
        }
        return ans;
    }
}
