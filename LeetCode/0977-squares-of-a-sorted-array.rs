impl Solution {
    fn decrease(o: &mut Option<usize>) {
        match *o {
            Some(i) => if i > 0 { o.replace(i - 1); } else { o.take(); },
            None => {}
        };
    }

    fn increase(o: &mut Option<usize>, max: usize) {
        match *o {
            Some(i) => if i < max - 1 { o.replace(i + 1); } else { o.take(); },
            None => {}
        }
    }

    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let z: Option<usize> = match nums.binary_search(&0) {
            Ok(i) => Some(i),
            Err(i) => if i < nums.len() { Some(i) } else { Some(i - 1) }
        };
        let mut l = z; Self::decrease(&mut l);
        let mut r = z;
        let mut ans: Vec<i32> = Vec::new();
        while l.is_some() && r.is_some() {
            let lv = nums[l.unwrap()];
            let rv = nums[r.unwrap()];
            if lv.abs() < rv.abs() {
                ans.push(lv * lv);
                Self::decrease(&mut l);
            } else {
                ans.push(rv * rv);
                Self::increase(&mut r, nums.len());
            }
        }
        while l.is_some() {
            let lv = nums[l.unwrap()];
            ans.push(lv * lv);
            Self::decrease(&mut l);
        }
        while r.is_some() {
            let rv = nums[r.unwrap()];
            ans.push(rv * rv);
            Self::increase(&mut r, nums.len());
        }
        ans
    }
}

