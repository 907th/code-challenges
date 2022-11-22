impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::enter(s) == Self::enter(t)
    }
    fn enter(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == '#' {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}
