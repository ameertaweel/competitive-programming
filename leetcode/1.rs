impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for i in 0..nums.len() {
            if h.contains_key(&nums[i]) && 2 * nums[i] == target {
                return vec![i as i32, h[&nums[i]]];
            }
            h.insert(nums[i], i as i32);
        }
        for a in h.keys() {
            if a + a == target {
                continue;
            }
            if h.contains_key(&(target - a)) {
                return vec![h[&a], h[&(target - a)]];
            }
        }
        panic!("No Solution. Assumption Violated.")
    }

    // Alternative Solution
    // Slower than the main solution
    pub fn two_sum_alt(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!("No Solution. Assumption Violated.")
    }
}
