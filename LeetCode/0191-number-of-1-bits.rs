impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut ans: i32 = 0;
        let mut n = n;
        while n > 0 {
            ans += (n & 1) as i32;
            n >>= 1;
        }
        ans
    }
}
