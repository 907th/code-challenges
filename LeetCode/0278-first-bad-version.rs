// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut l = 1i32;
        let mut r = n;
        while l < r {
            let c = l + (r - l) / 2;
            if self.isBadVersion(c) {
                r = c;
            } else {
                l = c + 1;
            }
        }
        l
    }
}
