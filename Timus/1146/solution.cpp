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
#include <cassert>

using namespace std;

int main() {
    const int N = 100;
    int n;
    int a[N][N];
    cin >> n;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++)
            cin >> a[i][j];
    int s[N][N];
    memset(s, 0, sizeof(s));
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++) {
            s[i][j] = a[i][j];
            if (i == 0 && j == 0) {
                // do nothing
            } else if (i == 0 && j > 0) {
                s[i][j] += s[i][j - 1];
            }
            else if (i > 0 && j == 0) {
                s[i][j] += s[i - 1][j];
            }
            else {
                s[i][j] += s[i][j - 1];
                s[i][j] += s[i - 1][j];
                s[i][j] -= s[i - 1][j - 1];
            }
        }
    int ans = 1 << 31;
    for (int i0 = 0; i0 < n; i0++)
        for (int j0 = 0; j0 < n; j0++)
            for (int i1 = i0; i1 < n; i1++)
                for (int j1 = j0; j1 < n; j1++) {
                    int cur = s[i1][j1];
                    if (i0 == 0 && j0 == 0) {
                        // do nothing
                    } else if (i0 == 0 && j0 > 0) {
                        cur -= s[i1][j0 - 1];
                    } else if (i0 > 0 && j0 == 0) {
                        cur -= s[i0 - 1][j1];
                    } else {
                        cur -= s[i1][j0 - 1];
                        cur -= s[i0 - 1][j1];
                        cur += s[i0 - 1][j0 - 1];
                    }
                    ans = max(ans, cur);
                }
    cout << ans << '\n';
    return 0;
}
