use std::collections::VecDeque;
type M = Vec<Vec<i32>>;
struct C(usize, usize);
impl Solution {
    pub fn update_matrix(mat: M) -> M {
        let mut ans = mat.clone();
        let (m, n) = (mat.len(), mat[0].len());
        for i in 0..m {
            for j in 0..n {
                ans[i][j] = -1;
            }
        }
        let mut queue: VecDeque<C> = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    ans[i][j] = 0;
                    queue.push_back(C(i, j));
                }
            }
        }
        while !queue.is_empty() {
            let C(i, j) = queue.pop_front().unwrap();
            if i > 0 && ans[i - 1][j] == -1 {
                ans[i - 1][j] = ans[i][j] + 1;
                queue.push_back(C(i - 1, j));
            }
            if i < m - 1 && ans[i + 1][j] == -1 {
                ans[i + 1][j] = ans[i][j] + 1;
                queue.push_back(C(i + 1, j));
            }
            if j > 0 && ans[i][j - 1] == -1 {
                ans[i][j - 1] = ans[i][j] + 1;
                queue.push_back(C(i, j - 1));
            }
            if j < n - 1 && ans[i][j + 1] == -1 {
                ans[i][j + 1] = ans[i][j] + 1;
                queue.push_back(C(i, j + 1));
            }
        }
        ans
    }
}
