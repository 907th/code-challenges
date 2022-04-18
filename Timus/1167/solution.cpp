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

typedef long long LL;

int main() {
    const int N = 500;
    int n, k;
    int c[N];
    cin >> n >> k;
    for (int i = 0; i < n; i++) cin >> c[i];

    // There are 2 parameters:
    // - number of stables used
    // - number of horses used
    LL dp[N + 1][N + 1];
    memset(dp, -1, sizeof(dp));
    dp[0][0] = 0;
    for (int ku = 0; ku < k; ku++)
        for (int hu = 0; hu < n; hu++) {
            if (dp[ku][hu] == -1) continue;
            int wc = 0;
            int bc = 0;
            for (int i = hu; i < n; i++) {
                if (c[i] == 1) bc++; else wc++;
                LL next = dp[ku + 1][hu + wc + bc];
                LL unhap = dp[ku][hu] + (LL) wc * bc;
                if (next == -1 || next > unhap) {
                    // printf("dp[%d][%d] = %lld\n", ku + 1, hu + wc + bc, unhap);
                    dp[ku + 1][hu + wc + bc] = unhap;
                }
            }
        }

    cout << dp[k][n] << '\n';
    return 0;
}
