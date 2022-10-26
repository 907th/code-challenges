struct Rec {
    n: i32,
    k: i32,
    ans: Vec<Vec<i32>>,
    cur: Vec<i32>
}

impl Rec {
    fn rec(&mut self, i: i32) {
        if self.cur.len() == self.k as usize {
            self.ans.push(self.cur.clone());
        } else if i <= self.n {
            // Take i
            {
                self.cur.push(i);
                self.rec(i + 1);
                self.cur.pop();
            }
            // Skip i
            {
                self.rec(i + 1);
            }
        }
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut rec = Rec { n, k, ans: Vec::new(), cur: Vec::new() };
        rec.rec(1);
        rec.ans
    }
}
