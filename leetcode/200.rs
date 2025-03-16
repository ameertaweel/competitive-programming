// LeetCode/200 - Number of Islands

use std::collections::HashMap;

struct DisjointSet {
    // Val -> (Parent, Size)
    map: HashMap<usize, (usize, usize)>,
}

impl DisjointSet {
    pub fn new() -> Self {
        DisjointSet {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: usize) {
        if self.map.contains_key(&val) {
            return;
        }
        self.map.insert(val, (val, 1));
    }

    pub fn find(&mut self, val: usize) -> usize {
        let mut root = val;
        // Find Root
        while self.map[&root].0 != root {
            root = self.map[&root].0;
        }
        // Compress Paths
        let mut node = val;
        while self.map[&node].0 != root {
            let parent = self.map[&node].0;
            self.map.get_mut(&node).unwrap().0 = root;
            node = parent;
        }
        return root;
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            // x and y are already in the same set
            return false;
        }

        let size_x = self.map[&x].1;
        let size_y = self.map[&y].1;

        let (x, y) = if size_x < size_y { (y, x) } else { (x, y) };

        self.map.get_mut(&y).unwrap().0 = x;
        self.map.get_mut(&x).unwrap().1 = size_x + size_y;

        return true;
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut disjoint_set = DisjointSet::new();
        let mut islands = 0;

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '0' {
                    continue;
                }

                let idx = r * cols + c;
                disjoint_set.insert(idx);
                islands += 1;

                let has_left = if c > 0 { grid[r][c - 1] == '1' } else { false };
                if has_left {
                    let idx_left = idx - 1;
                    if disjoint_set.union(idx, idx_left) {
                        islands -= 1;
                    }
                }

                let has_top = if r > 0 { grid[r - 1][c] == '1' } else { false };
                if has_top {
                    let idx_top = idx - cols;
                    if disjoint_set.union(idx, idx_top) {
                        islands -= 1;
                    }
                }
            }
        }

        return islands;
    }
}
