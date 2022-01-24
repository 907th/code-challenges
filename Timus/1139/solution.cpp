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
    int n, m;
    cin >> n >> m;
    n--;
    m--;
    int res = 0;
    const double EPS = 1e-10;
    for (int x1 = 1; x1 <= n; x1++) {
        int x0 = x1 - 1;
        int y0 = (int) ((double) m * x0 / n + EPS);
        int y1 = (int) ((double) m * x1 / n - EPS);
        res += y1 - y0 + 1;
    }
    cout << res << '\n';
    return 0;
}
