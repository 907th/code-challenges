impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut ans = 0u32;
        for i in 0..32 {
            ans = (ans << 1) | ((x >> i) & 1);
        }
        ans
    }
}
