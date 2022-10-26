struct Rec {
    nums: Vec<i32>,
    used: Vec<bool>,
    cur: Vec<i32>,
    ans: Vec<Vec<i32>>
}

impl Rec {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        Self {
            nums,
            used: vec![false; n],
            cur: Vec::new(),
            ans: Vec::new()
        }
    }

    fn rec(&mut self) {
        let n = self.nums.len();
        if self.cur.len() == n {
            self.ans.push(self.cur.clone());
            return;
        } else {
            for i in 0..n {
                if !self.used[i] {
                    self.used[i] = true;
                    self.cur.push(self.nums[i]);
                    self.rec();
                    self.cur.pop();
                    self.used[i] = false;
                }
            }
        }
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut rec = Rec::new(nums);
        rec.rec();
        rec.ans
    }
}
