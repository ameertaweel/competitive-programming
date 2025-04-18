// LeetCode/134 - Gas Station

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();

        let mut tnk = 0;
        let mut beg = 0;
        let mut cur = 0;

        loop {
            while tnk < 0 && beg < n {
                tnk -= gas[beg] - cost[beg];
                beg += 1;
            }
            if beg >= n {
                return -1;
            }
            tnk += gas[cur] - cost[cur];
            cur = (cur + 1) % n;
            if cur == beg && tnk >= 0 {
                return beg as i32;
            }
        }

        return -1;
    }
}
