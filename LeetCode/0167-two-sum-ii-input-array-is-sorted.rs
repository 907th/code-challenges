impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();
        let mut l = 0;
        let mut r = n - 1;
        loop {
            if l >= r { panic!("Unexpected situation!"); }
            let s = numbers[l] + numbers[r];
            if s == target { break; }
            else if s < target { l += 1; }
            else if s > target { r -= 1; }
        }
        vec![l as i32 + 1, r as i32 + 1]
    }
}

