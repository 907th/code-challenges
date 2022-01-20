#include <iostream>
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
#include <cassert>

using namespace std;

int main() {
    const int N = 1005;
    int n, m, k;
    bool diag[N][N];
    cin >> n >> m >> k;
    memset(diag, 0, sizeof(diag));
    for (int i = 0; i < k; i++) {
        int x, y;
        cin >> x >> y;
        diag[x][y] = true;
    }
    double ans[N][N];
    const double INF = 1e100;
    for (int x = 0; x <= n; x++)
        for (int y = 0; y <= m; y++)
            ans[x][y] = INF;
    ans[0][0] = 0.0;
    for (int x = 0; x <= n; x++)
        for (int y = 0; y <= m; y++) {
            if (y > 0) ans[x][y] = min(ans[x][y], ans[x][y - 1] + 1.0);
            if (x > 0) ans[x][y] = min(ans[x][y], ans[x - 1][y] + 1.0);
            if (diag[x][y]) ans[x][y] = min(ans[x][y], ans[x - 1][y - 1] + sqrt(2.0));
        }
    cout.setf(ios::fixed);
    cout.precision(0);
    cout << ans[n][m] * 100.0 << '\n';
    return 0;
}
