impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut chars_count = [0; 54];
        for c in s.chars() {
            if c.is_ascii_uppercase() {
                chars_count[((c as u8) - ('A' as u8)) as usize + 27] += 1;
            } else {
                chars_count[((c as u8) - ('a' as u8)) as usize] += 1;
            }
        }
        let mut odd = false;
        let mut ans = 0;
        for k in chars_count {
            if k & 1 == 1 {
                ans += k ^ 1;
                odd = true
            } else {
                ans += k;
            }
        }
        if odd { ans += 1; }
        ans
    }
}
