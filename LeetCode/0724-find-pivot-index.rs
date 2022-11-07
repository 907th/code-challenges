impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sums = nums.clone();
        let mut last = 0;
        for i in &mut sums {
            *i += last;
            last = *i;
        }
        let n = sums.len();
        for i in 0..sums.len() {
            let left = if i > 0 { sums[i - 1] } else { 0 };
            let right = sums[n - 1] - sums[i];
            if left == right { return i as i32; }
        }
        -1
    }
}
