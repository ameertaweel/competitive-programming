use std::collections::{HashMap, HashSet};

const TARGET: i32 = 0;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let nums = nums;

        let mut solutions = vec![];

        for (i, &ni) in nums.iter().enumerate() {
            if ni > 0 {
                break;
            }

            if i > 0 && ni == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;

            let target = TARGET - ni;

            while j < k {
                let nj = nums[j];
                let nk = nums[k];

                if nj + nk < target {
                    j += 1;
                    continue;
                }
                if nj + nk > target {
                    k -= 1;
                    continue;
                }
                solutions.push(vec![ni, nj, nk]);
                j += 1;
                k -= 1;
                while nums[j - 1] == nums[j] && j < k {
                    j += 1;
                }
            }
        }

        return solutions;
    }

    pub fn three_sum_alt(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Store the multiplicities of each number
        let mut m = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            *m.entry(n).or_insert(0) += 1;
        }
        let m = m;

        let nums: HashSet<_> = nums.iter().collect();

        let mut solutions = HashSet::new();

        for &&ni in nums.iter() {
            let mi = m[&ni];
            for &&nj in nums.iter() {
                if ni > nj {
                    continue;
                }
                if ni == nj && m[&ni] < 2 {
                    continue;
                }
                let nk = TARGET - ni - nj;
                if !m.contains_key(&nk) {
                    continue;
                }
                if ni == nj && nj == nk && mi < 3 {
                    continue;
                }
                if ni == nk && mi < 2 {
                    continue;
                }
                let mk = m[&nk];
                if nj == nk && mk < 2 {
                    continue;
                }

                let answer = {
                    let (a, b) = if ni < nj { (ni, nj) } else { (nj, ni) };
                    if nk < a {
                        (nk, a, b)
                    } else if nk < b {
                        (a, nk, b)
                    } else {
                        (a, b, nk)
                    }
                };
                solutions.insert(answer);
            }
        }

        return solutions.iter().map(|&(a, b, c)| vec![a, b, c]).collect();
    }
}
