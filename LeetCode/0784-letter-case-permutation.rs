impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let n = s.len();
        let c: Vec<char> = s.chars().collect();
        'main_loop: for i in 0..(1 << n) {
            let mut s = String::new();
            for j in 0..n {
                if c[j].is_alphabetic() {
                    if (i & (1 << j)) > 0 {
                        s.push_str(&c[j].to_uppercase().to_string());
                    } else {
                        s.push_str(&c[j].to_lowercase().to_string());
                    }
                } else {
                    if (i & (1 << j)) > 0 {
                        continue 'main_loop;
                    } else {
                        s.push(c[j]);
                    }
                }
            }
            ans.push(s);
        }
        ans
    }
}
