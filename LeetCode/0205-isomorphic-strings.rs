impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut hs: [Option<char>; 256] = [None; 256];
        let mut ht: [Option<char>; 256] = [None; 256];
        let mut z = s.chars().zip(t.chars());
        for (s, t) in z {
            if let Some(x) = hs[s as usize] {
                if x != t { return false; }
            } else {
                if ht[t as usize] == None {
                    hs[s as usize] = Some(t);
                    ht[t as usize] = Some(s);
                } else {
                    return false;
                }
            }
        }
        true
    }
}
