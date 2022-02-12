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

int main() {
    int dp[10][82];
    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;
    for (int i = 1; i <= 9; i++)
        for (int j = 0; j <= 81; j++)
            for (int k = 0; k <= 9; k++) {
                if (j - k < 0) continue;
                dp[i][j] += dp[i - 1][j - k];
            }
    int s;
    cin >> s;
    int ans = dp[9][s];
    if (s == 1) ans++;
    cout << ans << '\n';
    return 0;
}
