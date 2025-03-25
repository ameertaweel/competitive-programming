// LeetCode/207 - Course Schedule

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;

        let mut needs: Vec<Vec<usize>> = vec![vec![]; n];
        let mut needed_by: Vec<Vec<usize>> = vec![vec![]; n];
        for p in prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            if a == b {
                return false;
            }
            needs[a].push(b);
            needed_by[b].push(a);
        }

        let mut seen_perm = vec![false; n];

        for i in 0..n {
            if needed_by[i].len() > 0 {
                continue;
            }
            if seen_perm[i] {
                continue;
            }

            let mut seen_temp = vec![false; n];
            seen_perm[i] = true;
            seen_temp[i] = true;

            if !Self::can_finish_helper(i, &needs, &mut seen_perm, &mut seen_temp) {
                return false;
            }
        }

        for s in seen_perm {
            if !s {
                return false;
            }
        }

        return true;
    }

    pub fn can_finish_helper(
        i: usize,
        needs: &Vec<Vec<usize>>,
        seen_perm: &mut Vec<bool>,
        seen_temp: &mut Vec<bool>,
    ) -> bool {
        for &j in &needs[i] {
            if seen_temp[j] {
                return false;
            }
            if seen_perm[j] {
                continue;
            }
            seen_perm[j] = true;
            seen_temp[j] = true;
            if !Self::can_finish_helper(j, needs, seen_perm, seen_temp) {
                return false;
            }
            seen_temp[j] = false;
        }
        return true;
    }
}
