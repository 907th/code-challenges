impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut k = 0usize;
        for i in 0..n {
            if nums[i] != 0 {
                nums[i - k] = nums[i];
            } else {
                k += 1;
            }
        }
        for i in 0..k {
            nums[n - i - 1] = 0;
        }
    }
}

