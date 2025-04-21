// LeetCode/763 - Partition Labels

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let chars: Vec<_> = s.chars().map(|c| (c as usize) - ('a' as usize)).collect();
        let n = chars.len();

        let mut pos_last = vec![0; 26];
        for i in 0..n {
            let c = chars[i];
            pos_last[c] = i;
        }
        let pos_last = pos_last;

        let mut partitions = vec![];
        let mut curr = 0;
        let mut till = 0;

        while curr < n {
            let c = chars[curr];
            if curr == till {
                partitions.push(1);
                till = pos_last[c] + 1;
            } else {
                let last = partitions.len() - 1;
                partitions[last] += 1;
                if pos_last[c] + 1 > till {
                    till = pos_last[c] + 1;
                }
            }
            curr += 1;
        }

        return partitions;
    }
}
