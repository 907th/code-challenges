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

// Use Prim algo to solve this problem
int main() {
    int n, k;
    int c[100][100];
    int e[100];
    cin >> n >> k;
    for (int i = 0; i < k; i++) {
        cin >> e[i];
        e[i]--;
    }
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++) cin >> c[i][j];
    for (int i = 0; i < k; i++)
        for (int j = 0; j < k; j++) c[e[i]][e[j]] = 0;

    int ans = 0;
    bool used[100];
    memset(used, 0, sizeof(used));
    used[0] = true;
    for (int o = 1; o < n; o++) {
        int next = -1;
        int cost = 1 << 30;
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (used[i] && !used[j])
                    if (next == -1 || c[i][j] < cost) {
                        next = j;
                        cost = c[i][j];
                    }
        used[next] = true;
        ans += cost;
    }
    cout << ans << '\n';
    return 0;
}
