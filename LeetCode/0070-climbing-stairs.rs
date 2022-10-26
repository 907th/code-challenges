impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut ans = vec![0; n + 1];
        ans[0] = 1;
        for i in 0..n {
            ans[i + 1] += ans[i];
            if i + 2 <= n { ans[i + 2] += ans[i]; }
        }
        ans[n]
    }
}
