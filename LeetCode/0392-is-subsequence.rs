impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut si = s.chars();
        let mut so = si.next();
        for tc in t.chars() {
            if let Some(sc) = so {
                if tc == sc { so = si.next() }
            }
        }
        so == None
    }
}
