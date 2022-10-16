impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0usize;
        let mut r = s.len() - 1;
        while l < r {
            let c = s[l];
            s[l] = s[r];
            s[r] = c;
            l += 1; r -= 1;
        }
    }
}

