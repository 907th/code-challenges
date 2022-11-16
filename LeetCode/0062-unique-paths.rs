impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0usize; n]; m];
        dp[0][0] = 1;
        for i in 0..m {
            for j in 0..n {
                if i > 0 { dp[i][j] += dp[i - 1][j]; }
                if j > 0 { dp[i][j] += dp[i][j - 1]; }
            }
        }
        dp[m - 1][n - 1] as i32
    }
}
