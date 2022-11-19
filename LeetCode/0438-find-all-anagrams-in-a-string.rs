impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() { return vec![]; }

        let mut s: Vec<usize> = s.chars().map(|c| (c as u8 - 'a' as u8) as usize).collect();
        let mut p: Vec<usize> = p.chars().map(|c| (c as u8 - 'a' as u8) as usize).collect();

        let mut pk = [0usize; 27];
        for &v in p.iter() { pk[v] += 1; }

        let mut ok_chars = 0;
        let mut sk = [0usize; 27];
        for &v in s[0..p.len()].iter() {
            if pk[v] > sk[v] { ok_chars += 1; }
            sk[v] += 1;
        }

        let mut ans: Vec<i32> = Vec::new();
        if ok_chars == p.len() { ans.push(0); }
        for i in 0..(s.len() - p.len()) {
            let (vl, vr) = (s[i], s[i + p.len()]);
            if pk[vl] >= sk[vl] { ok_chars -= 1; }
            sk[vl] -= 1;
            if pk[vr] > sk[vr] { ok_chars += 1; }
            sk[vr] += 1;
            if ok_chars == p.len() { ans.push((i + 1) as i32); }
        }
        ans
    }
}
