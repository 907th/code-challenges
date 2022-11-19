impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        nums.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
        let mut i = 0usize;
        let mut j = nums.len() - 1;
        loop {
            if i >= j { panic!("Solution must exist!"); }
            let sum = nums[i].1 + nums[j].1;
            if sum == target { break; }
            if sum < target { i += 1; } else { j -= 1; }
        }
        vec![nums[i].0 as i32, nums[j].0 as i32]
    }
}
