#include <iostream>
#include <iomanip>
#include <algorithm>
#include <vector>
#include <array>
#include <set>
#include <map>
#include <unordered_map>
#include <queue>
#include <stack>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cctype>
#include <cassert>

using namespace std;

int main() {
    int n, k;
    cin >> n >> k;
    long long dp[20][2];
    dp[0][0] = 0;
    dp[0][1] = k - 1;
    for (int i = 1; i < n; i++) {
        dp[i][0] = dp[i - 1][1];
        dp[i][1] = (dp[i - 1][0] + dp[i - 1][1]) * (k - 1);
    }
    long long ans = dp[n - 1][0] + dp[n - 1][1];
    cout << ans << '\n';
    return 0;
}
