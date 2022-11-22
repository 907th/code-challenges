impl Solution {
    pub fn decode_string(s: String) -> String {
        let v: Vec<char> = s.chars().collect();
        Self::parse(&v[..], 0).0
    }

    fn parse(s: &[char], mut i: usize) -> (String, usize) {
        let mut res = String::new();
        while i < s.len() {
            if s[i].is_ascii_digit() {
                let (u, j) = Self::repeat(s, i);
                res = res + &u;
                i = j;
            } else if s[i] == ']' {
                break;
            } else {
                res = res + &String::from(s[i]);
                i = i + 1;
            }
        }
        (res, i)
    }

    fn repeat(s: &[char], mut i: usize) -> (String, usize) {
        let mut res = String::new();

        let mut j = i;
        while s[j].is_ascii_digit() { j = j + 1; }
        let k: usize = s[i..j].iter().collect::<String>().parse().unwrap();

        i = j;
        assert!(s[i] == '[');
        i = i + 1;

        let (u, j) = Self::parse(s, i);
        for _ in 0..k { res = res + &u; }

        i = j;
        assert!(s[i] == ']');
        i = i + 1;

        (res, i)
    }
}
