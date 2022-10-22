use std::collections::VecDeque;
use std::cmp::max;
type G = Vec<Vec<i32>>;
struct C(usize, usize);
impl Solution {
    pub fn oranges_rotting(grid: G) -> i32 {
        let mut g = grid.clone();
        let (m, n) = (g.len(), g[0].len());
        let mut q: VecDeque<C> = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if g[i][j] == 2 {
                    q.push_back(C(i, j));
                }
            }
        }
        while !q.is_empty() {
            let C(i, j) = q.pop_front().unwrap();
            if i > 0 && g[i - 1][j] == 1 {
                g[i - 1][j] = g[i][j] + 1;
                q.push_back(C(i - 1, j));
            }
            if i < m - 1 && g[i + 1][j] == 1 {
                g[i + 1][j] = g[i][j] + 1;
                q.push_back(C(i + 1, j));
            }
            if j > 0 && g[i][j - 1] == 1 {
                g[i][j - 1] = g[i][j] + 1;
                q.push_back(C(i, j - 1));
            }
            if j < n - 1 && g[i][j + 1] == 1 {
                g[i][j + 1] = g[i][j] + 1;
                q.push_back(C(i, j + 1));
            }
        }
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if g[i][j] == 1 { return -1; }
                if g[i][j] > 1 { ans = max(ans, g[i][j] - 2); }
            }
        }
        ans
    }
}
