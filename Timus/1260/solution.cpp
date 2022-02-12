#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <deque>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

// Naive solution (BF)

bool valid(const vector<int> &v) {
    if (v[0] != 1) return false;
    for (int i = 1; i < v.size(); i++)
        if (abs(v[i] - v[i - 1]) > 2) return false;
    return true;
}

void print(const vector<int> &v) {
    for (int i = 0; i < v.size(); i++) {
        cout << v[i];
        cout << (i < v.size() - 1 ? ' ' : '\n');
    }
}

int solve_naive(int n) {
    int ans = 0;
    vector<int> v;
    for (int i = 1; i <= n; i++) v.push_back(i);
    while (1) {
        if (valid(v)) {
            print(v);
            ans++;
        }
        if (!next_permutation(v.begin(), v.end())) break;
    }
    return ans;
}

// Fast algo (DP)

const int N = 56;

int dp[N][N];
int ans[N];

void precalc_fast() {
    memset(dp, 0, sizeof(dp));
    for (int n = 1; n < N; n++) {
        for (int i = 0; true; i++) { // place number n to position (n - i)
            int k = 2 * i + 1; // place i numbers to the left and to the right of number n
            if (k > n) break;

            int odd_last = n - max(2 * i - 1, 0);
            int even_last = n - (2 * i);
            int left_pos = n - k + 1;

            int left_val, right_val;

            // Place numbers (n - 1), (n - 3), etc to the right of number n;
            // and (n - 2), (n - 4), etc - to the left.
            left_val = even_last;
            right_val = odd_last;
            if (left_pos == 1) {
                assert(left_val == 1);
                dp[n][right_val] = 1;
            } else { // left_pos > 1
                assert(left_val > 1);
                dp[n][right_val] =
                    (
                        dp[left_pos - 1][left_val - 1] +
                        (left_val > 2 ? dp[left_pos - 1][left_val - 2] : 0)
                    );
            }

            if (i == 0) continue;

            // Place numbers (n - 1), (n - 3), etc to the left of number n;
            // and (n - 2), (n - 4), etc - to the right.
            left_val = odd_last;
            right_val = even_last;
            if (left_pos == 1) {
                assert(left_val == 2);
            } else { // left_pos > 1
                assert(left_val > 2);
                dp[n][right_val] = dp[left_pos - 1][left_val - 2];
            }
        }
    }

    memset(ans, 0, sizeof(ans));
    for (int i = 1; i < N; i++) {
        for (int j = 1; j <= i; j++) {
            ans[i] += dp[i][j];
        }
    }
}

int solve_fast(int n) {
    return ans[n];
}


// Main program

const bool CHECK_NAIVE = false;

int solve(int n) {
    int fast = solve_fast(n);
    if (CHECK_NAIVE) {
        int naive = solve_naive(n);
        cout << "Naive: " << naive << '\n';
        cout << "Fast: " << fast << '\n';
    }
    return fast;
}

int main() {
    precalc_fast();
    int n;
    cin >> n;
    cout << solve(n) << '\n';
    return 0;
}
