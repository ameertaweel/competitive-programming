use std::collections::HashMap;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut candidates = nums.clone();
        candidates.sort_unstable();
        candidates.dedup();
        candidates.reverse();
        let mut multiplicities = HashMap::new();
        for &n in &nums {
            *multiplicities.entry(n).or_insert(0) += 1;
        }
        let mut answers = vec![vec![]];
        let mut partial = vec![];
        Self::subsets_with_dup_helper(&candidates, &mut multiplicities, &mut answers, &mut partial);
        return answers;
    }

    pub fn subsets_with_dup_helper(
        nums: &Vec<i32>,
        multiplicities: &mut HashMap<i32, i32>,
        answers: &mut Vec<Vec<i32>>,
        partial: &mut Vec<i32>,
    ) {
        // Uses the fact that each number is at least -10 (question constraints)
        let max = *partial.last().unwrap_or(&-11);
        for &n in nums {
            if n < max {
                break;
            }
            if multiplicities[&n] == 0 {
                continue;
            }
            *multiplicities.get_mut(&n).unwrap() -= 1;
            partial.push(n);
            answers.push(partial.clone());
            Self::subsets_with_dup_helper(nums, multiplicities, answers, partial);
            partial.pop().unwrap();
            *multiplicities.get_mut(&n).unwrap() += 1;
        }
    }
}
