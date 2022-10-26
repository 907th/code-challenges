use std::cmp::min;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut t = triangle;
        let n = t.len();
        for l in (0..(n - 1)).rev() {
            for i in 0..=l {
                t[l][i] = min(t[l][i] + t[l + 1][i], t[l][i] + t[l + 1][i + 1]);
            }
        }
        t[0][0]
    }
}
