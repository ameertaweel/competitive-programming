use std::collections::HashMap;

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
struct TimeMap {
    h: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap { h: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.h.entry(key).or_insert(vec![]).push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        // Ensure key exists in hash map
        if !self.h.contains_key(&key) {
            return "".to_string();
        }

        let v = &self.h[&key];

        let mut beg = 0;
        let mut end = v.len();

        let mut ans_t = 0;
        let mut ans_i = 0;

        while beg < end {
            let mid = beg + (end - beg) / 2;
            let t = v[mid].0;

            if t == timestamp {
                return v[mid].1.clone();
            }

            if t > timestamp {
                end = mid;
                continue;
            }

            if t > ans_t {
                ans_t = t;
                ans_i = mid;
                beg = mid + 1;
            }
        }

        return if ans_t > 0 {
            v[ans_i].1.clone()
        } else {
            "".to_string()
        };
    }
}
