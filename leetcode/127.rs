// LeetCode/127 - Word Ladder

use std::collections::{BTreeSet, HashMap, HashSet};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_list = {
            let mut word_list = word_list;
            word_list.push(begin_word.clone());
            word_list.sort_unstable();
            word_list.dedup();
            word_list
        };

        let (begin_idx, end_idx) = {
            let (mut begin_idx, mut end_idx) = (word_list.len(), word_list.len());
            for i in 0..word_list.len() {
                if word_list[i] == begin_word {
                    begin_idx = i;
                }
                if word_list[i] == end_word {
                    end_idx = i;
                }
                if begin_idx < word_list.len() && end_idx < word_list.len() {
                    break;
                }
            }
            (begin_idx, end_idx)
        };

        // There is no way to reach end_word
        if end_idx == word_list.len() {
            return 0;
        }

        let graph = {
            let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
            for i in 0..word_list.len() {
                graph.insert(i, HashSet::new());
            }
            for i in 0..word_list.len() {
                for j in (i + 1)..word_list.len() {
                    if word_list[i].diff_chars_count(&word_list[j]) == 1 {
                        graph.get_mut(&i).unwrap().insert(j);
                        graph.get_mut(&j).unwrap().insert(i);
                    }
                }
            }
            graph
        };

        let mut d = vec![usize::MAX; graph.len()];
        let mut q = BTreeSet::new();

        d[begin_idx] = 0;
        q.insert((0, begin_idx));

        while q.len() > 0 {
            let (_, idx) = q.pop_first().unwrap();

            for &out_edge in &graph[&idx] {
                if d[idx] + 1 < d[out_edge] {
                    q.remove(&(d[out_edge], out_edge));
                    d[out_edge] = d[idx] + 1;
                    q.insert((d[out_edge], out_edge));
                }
            }
        }

        // There is no way to reach end_word
        if d[end_idx] == usize::MAX {
            return 0;
        }

        return (d[end_idx] as i32) + 1;
    }
}

trait DiffCount {
    fn diff_chars_count(&self, other: &str) -> usize;
}

impl DiffCount for str {
    fn diff_chars_count(&self, other: &str) -> usize {
        assert_eq!(self.len(), other.len(), "Strings must have same length");

        self.as_bytes()
            .iter()
            .zip(other.as_bytes())
            .filter(|(a, b)| a != b)
            .count()
    }
}
