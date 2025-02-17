use std::collections::HashMap;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = candidates.clone();
        nums.sort_unstable();
        nums.dedup();
        nums.reverse();
        let mut answers = vec![];
        let mut partial = vec![];
        let mut multiplicities = HashMap::new();

        for &n in &candidates {
            *multiplicities.entry(n).or_insert(0) += 1;
        }

        combination_sum2_helper(
            &nums,
            &mut multiplicities,
            target,
            &mut answers,
            &mut partial,
            0,
        );

        return answers;
    }
}

pub fn combination_sum2_helper(
    nums: &Vec<i32>,
    multiplicities: &mut HashMap<i32, i32>,
    target: i32,
    answers: &mut Vec<Vec<i32>>,
    partial: &mut Vec<i32>,
    sum: i32,
) {
    if sum > target {
        return;
    }
    if sum == target {
        answers.push(partial.clone());
        return;
    }

    // Uses the fact that each number is at least 1 (question constraints)
    let max = *partial.last().unwrap_or(&0);

    for &n in nums {
        if n < max {
            break;
        }
        if multiplicities[&n] == 0 {
            continue;
        }
        *multiplicities.get_mut(&n).unwrap() -= 1;
        partial.push(n);
        combination_sum2_helper(&nums, multiplicities, target, answers, partial, sum + n);
        partial.pop().unwrap();
        *multiplicities.get_mut(&n).unwrap() += 1;
    }
}
