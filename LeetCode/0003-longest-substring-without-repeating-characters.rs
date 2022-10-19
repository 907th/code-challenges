use std::cmp::{max, min};

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = [-1; 127];
        let mut left = -1;
        let mut ans = 0;
        for (i, c) in s.chars().enumerate() {
            assert!(c.is_ascii(), "Not an ASCII character!");
            left = max(left, last[c as usize]);
            ans = max(ans, i as i32 - left);
            last[c as usize] = i as i32;
        }
        ans
    }
}

