impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0i32;
        for x in nums {
            ans = ans ^ x;
        }
        ans
    }
}
