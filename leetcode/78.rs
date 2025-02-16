use std::collections::HashMap;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answers = vec![vec![]];
        let mut partial = vec![];
        let mut original_multiplicities = HashMap::new();
        let mut partial_multiplicities = HashMap::new();

        for &n in &nums {
            *original_multiplicities.entry(n).or_insert(0) += 1;
            partial_multiplicities.insert(n, 0);
        }

        subsets_helper(
            &nums,
            &mut answers,
            &mut partial,
            &original_multiplicities,
            &mut partial_multiplicities,
        );

        return answers;
    }
}

pub fn subsets_helper(
    nums: &Vec<i32>,
    answers: &mut Vec<Vec<i32>>,
    partial: &mut Vec<i32>,
    original_multiplicities: &HashMap<i32, i32>,
    partial_multiplicities: &mut HashMap<i32, i32>,
) {
    // Uses the fact that each number is at least -10 (question constraints)
    let max = *partial.last().unwrap_or(&-11);

    for &n in nums {
        if n < max {
            continue;
        }
        if partial_multiplicities[&n] == original_multiplicities[&n] {
            continue;
        }
        *partial_multiplicities.get_mut(&n).unwrap() += 1;
        partial.push(n);
        answers.push(partial.clone());
        subsets_helper(
            nums,
            answers,
            partial,
            original_multiplicities,
            partial_multiplicities,
        );
        partial.pop().unwrap();
        *partial_multiplicities.get_mut(&n).unwrap() -= 1;
    }
}
