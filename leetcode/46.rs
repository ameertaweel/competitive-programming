use std::collections::HashSet;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut candidates = nums.clone().into_iter().collect();
        let mut answers = vec![];
        let mut partial = vec![];

        Self::permute_helper(&nums, &mut candidates, &mut answers, &mut partial);

        return answers;
    }

    pub fn permute_helper(
        nums: &Vec<i32>,
        candidates: &mut HashSet<i32>,
        answers: &mut Vec<Vec<i32>>,
        partial: &mut Vec<i32>,
    ) {
        if candidates.len() == 0 {
            answers.push(partial.clone());
            return;
        }

        for &n in nums {
            if !candidates.contains(&n) {
                continue;
            }
            candidates.remove(&n);
            partial.push(n);
            Self::permute_helper(nums, candidates, answers, partial);
            partial.pop().unwrap();
            candidates.insert(n);
        }
    }
}
