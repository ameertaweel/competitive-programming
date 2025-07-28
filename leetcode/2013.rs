// LeetCode/2013 - Detect Squares

use std::collections::{HashMap, HashSet};

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
struct DetectSquares {
    points: HashMap<(i32, i32), i32>,
    pointsx: HashMap<i32, HashSet<i32>>,
    squares: HashMap<(i32, i32), i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    fn new() -> Self {
        DetectSquares {
            points: HashMap::new(),
            pointsx: HashMap::new(),
            squares: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let (x, y) = (point[0], point[1]);
        *self.points.entry((x, y)).or_default() += 1;
        self.pointsx.entry(x).or_default().insert(y);
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (x, y) = (point[0], point[1]);
        let mut ans = 0;
        if self.pointsx.contains_key(&x) {
            for &other_y in &self.pointsx[&x] {
                if other_y == y {
                    continue;
                }
                let diff = other_y - y;
                ans += *self.points.get(&(x, other_y)).unwrap_or(&0)
                    * *self.points.get(&(x - diff, other_y)).unwrap_or(&0)
                    * *self.points.get(&(x - diff, y)).unwrap_or(&0);
                ans += *self.points.get(&(x, other_y)).unwrap_or(&0)
                    * *self.points.get(&(x + diff, other_y)).unwrap_or(&0)
                    * *self.points.get(&(x + diff, y)).unwrap_or(&0);
            }
        }
        return ans;
    }
}
