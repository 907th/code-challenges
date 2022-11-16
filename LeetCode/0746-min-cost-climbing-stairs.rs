use std::cmp::min;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut ans = vec![i32::MAX; n];
        ans[0] = cost[0];
        ans[1] = cost[1];
        for i in 2..n {
            ans[i] = min(ans[i - 1], ans[i - 2]) + cost[i];
        }
        min(ans[n - 1], ans[n - 2])
    }
}
