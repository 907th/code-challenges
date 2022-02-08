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

const int P = 1000000000 + 7;

typedef int ary[2][301];

int main() {
    int n, a, b;
    cin >> n >> a >> b;

    ary x, y;
    memset(x, 0, sizeof(x));
    x[0][1] = 1;
    x[1][1] = 1;

    ary *p1 = &x, *p2 = &y;
    for (int i = 2; i <= n; i++) {
        memset(*p2, 0, sizeof(*p2));

        for (int j = 1; j <= max(a, b); j++) {
            if (j <= a) (*p2)[1][1] = ((long long) (*p2)[1][1] + (*p1)[0][j]) % P;
            if (j <= b) (*p2)[0][1] = ((long long) (*p2)[0][1] + (*p1)[1][j]) % P;
        }

        for (int j = 2; j <= max(a, b); j++) {
            if (j <= a) (*p2)[0][j] = (*p1)[0][j - 1];
            if (j <= b) (*p2)[1][j] = (*p1)[1][j - 1];
        }

        swap(p1, p2);
    }

    int ans = 0;
    for (int j = 1; j <= max(a, b); j++) {
        ans = ((long long) ans + (*p1)[0][j]) % P;
        ans = ((long long) ans + (*p1)[1][j]) % P;
    }
    cout << ans << '\n';

    return 0;
}
