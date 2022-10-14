impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match (nums.binary_search(&target)) {
            Ok(i) => i as i32,
            _ => -1
        }
    }
}

/* OR

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len();
        while l < r {
            let c: usize = (l + r) / 2;
            if nums[c] < target { l = c + 1; } else { r = c; }
        }
        if l < nums.len() && nums[l] == target { l as i32 } else { -1 }
    }
}

*/
