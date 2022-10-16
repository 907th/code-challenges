impl Solution {
    pub fn reverse_words(s: String) -> String {
        s
            .split_whitespace()
            .map(|w| w.chars().rev().collect::<String>())
            .reduce(|f, w| if f.len() > 0 { f + " " + &w } else { String::from(w) })
            .unwrap()
    }
}

