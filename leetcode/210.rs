// LeetCode/210 - Course Schedule II

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;

        let mut needs: Vec<usize> = vec![0; n];
        let mut needed_by: Vec<Vec<usize>> = vec![vec![]; n];
        for p in prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            if a == b {
                return vec![];
            }
            needs[a] += 1;
            needed_by[b].push(a);
        }

        let mut stack = vec![];
        for i in 0..n {
            if needs[i] == 0 {
                stack.push(i);
            }
        }

        let mut order = vec![];

        while stack.len() > 0 {
            let i = stack.pop().unwrap();
            order.push(i as i32);

            for &j in &needed_by[i] {
                needs[j] -= 1;
                if needs[j] == 0 {
                    stack.push(j);
                }
            }
        }

        if order.len() == n {
            return order;
        }

        return vec![];
    }
}
