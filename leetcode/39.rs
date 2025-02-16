impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut answers = vec![];
        let mut partial = vec![];
        combination_sum_helper(&candidates, target, &mut answers, &mut partial, 0);
        return answers;
    }
}

pub fn combination_sum_helper(
    candidates: &Vec<i32>,
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

    // Uses the fact that candidates are at least 2 (question constraints)
    let max = *partial.last().unwrap_or(&1);
    for &c in candidates {
        if c < max {
            continue;
        }
        partial.push(c);
        combination_sum_helper(candidates, target, answers, partial, sum + c);
        partial.pop().unwrap();
    }
}
