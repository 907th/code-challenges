impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<usize> = s.chars().map(|c| (c as u8 - 'A' as u8) as usize).collect();
        let n = s.len();
        let mut ans = 0usize;
        let mut cnt = [0usize; 27];
        let mut i = 0usize;
        let mut j = i;
        while i < n {
            let max = cnt.iter().max().unwrap_or(&0);
            let sum: usize = cnt.iter().sum();
            let ok = (sum - max) <= k as usize;
            if ok { ans = std::cmp::max(ans, j - i); }
            if ok && j < n {
                cnt[s[j]] += 1;
                j += 1;
            } else {
                cnt[s[i]] -= 1;
                i += 1;
            }
        }
        ans as i32
    }
}
