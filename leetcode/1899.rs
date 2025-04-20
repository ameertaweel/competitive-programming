// LeetCode/1899 - Merge Triplets to Form Target Triplet

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut found = [false, false, false];
        for t in &triplets {
            if t[0] > target[0] || t[1] > target[1] || t[2] > target[2] {
                continue;
            }
            found[0] |= t[0] == target[0];
            found[1] |= t[1] == target[1];
            found[2] |= t[2] == target[2];
            if found[0] && found[1] && found[2] {
                return true;
            }
        }
        return false;
    }
}
