impl Solution {
    // Time  Complexity: O(N)
    // Space Complexity: O(1)
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut suffix = 1;
        for i in (0..nums.len()).rev() {
            ans[i] = suffix;
            suffix *= nums[i];
        }
        let mut prefix = 1;
        for i in 0..nums.len() {
            ans[i] *= prefix;
            prefix *= nums[i];
        }
        return ans;
    }

    // Time  Complexity: O(N)
    // Space Complexity: O(N)
    pub fn product_except_self_alt(nums: Vec<i32>) -> Vec<i32> {
        let prefixes = {
            let mut vec = vec![0; nums.len()];
            let mut acc = 1;
            for i in 0..nums.len() {
                vec[i] = acc;
                acc *= nums[i];
            }
            vec
        };
        let suffixes = {
            let mut vec = vec![0; nums.len()];
            let mut acc = 1;
            for i in (0..nums.len()).rev() {
                vec[i] = acc;
                acc *= nums[i];
            }
            vec
        };
        return prefixes
            .iter()
            .zip(suffixes.iter())
            .map(|(p, s)| p * s)
            .collect();
    }
}
