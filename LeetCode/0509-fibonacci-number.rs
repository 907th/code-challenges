impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut nums: [i32; 31] = [0; 31];
        nums[0] = 0;
        nums[1] = 1;
        for i in 2..31usize {
            nums[i] = nums[i - 1] + nums[i - 2];
        }
        nums[n as usize]
    }
}
