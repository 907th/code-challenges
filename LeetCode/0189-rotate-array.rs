use std::mem;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if k == 0 { return; }
        let n = nums.len();
        let g = Self::gcd(k as usize, n);
        for i in 0..g {
            let mut z = nums[i];
            let mut j = (i + k as usize) % n;
            while j != i {
                mem::swap(&mut z, &mut nums[j]);
                j = (j + k as usize) % n;
            }
            nums[i] = z;
        }
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
}
