// LeetCode/684 - Redundant Connection

// UnionFind data structure
// With path compression and union by rank optimizations
struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut root = vec![0; size];
        for i in 0..size {
            root[i] = i;
        }
        Self {
            root,
            rank: vec![1; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.root[x] {
            return x;
        }
        // Path Compression
        self.root[x] = self.find(self.root[x]);
        return self.root[x];
    }

    fn union(&mut self, x: usize, y: usize) {
        let rootX = self.find(x);
        let rootY = self.find(y);

        if rootX != rootY {
            // Union By Rank
            if self.rank[rootX] > self.rank[rootY] {
                self.root[rootY] = rootX;
            } else if self.rank[rootX] < self.rank[rootY] {
                self.root[rootX] = rootY;
            } else {
                self.root[rootY] = rootX;
                self.rank[rootX] += 1;
            }
        }
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        return self.find(x) == self.find(y);
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = {
            let mut n = 0;
            for e in &edges {
                let &[a, b] = e.as_slice() else {
                    unreachable!()
                };
                let (a, b) = (a as usize, b as usize);
                if a > n {
                    n = a;
                }
                if b > n {
                    n = b;
                }
            }
            n
        };

        let mut uf = UnionFind::new(n);

        for e in edges {
            let &[a, b] = e.as_slice() else {
                unreachable!()
            };
            let (a, b) = (a as usize - 1, b as usize - 1);
            if uf.connected(a, b) {
                return e;
            }
            uf.union(a, b);
        }

        return vec![];
    }
}
