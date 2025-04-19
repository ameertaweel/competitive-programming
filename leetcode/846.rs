// LeetCode/846 - Hand of Straights

use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;

        if hand.len() % group_size != 0 {
            return false;
        }

        let mut mult = HashMap::new();
        for v in &hand {
            if !mult.contains_key(v) {
                mult.insert(*v, 0);
            }
            *mult.get_mut(v).unwrap() += 1;
        }

        let mut hand = hand;
        hand.sort_unstable();
        hand.dedup();
        let hand = hand;

        for i in 0..hand.len() {
            let ak = hand[i];
            let av = mult[&ak];
            if av == 0 {
                continue;
            }
            for j in 0..group_size {
                match mult.get_mut(&(ak + j as i32)) {
                    None => return false,
                    Some(m) => {
                        if *m < av {
                            return false;
                        }
                        *m -= av;
                    }
                }
            }
        }

        return true;
    }
}
