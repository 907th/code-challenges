impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut last = 0;
        for v in &mut nums {
            *v += last;
            last = *v;
        }
        nums
    }
}
