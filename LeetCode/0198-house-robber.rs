use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut m = vec![0 as i32; n];
        m[0] = nums[0];
        for i in 1..n {
            m[i] = m[i - 1];
            if i >= 2 {
                m[i] = max(m[i], nums[i] + m[i - 2]);
            } else {
                m[i] = max(m[i], nums[i]);
            }
        }
        m[n - 1]
    }
}
