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
    int x, n;
    cin >> n >> x;
    const int INF = 1 << 30;
    int left = -INF, right = INF;
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v;
        if (v > 0) right = min(right, v);
        if (v < 0) left = max(left, v);
    }
    if ((right != INF && x > right) || (left != -INF && x < left)) {
        cout << "Impossible\n";
    } else {
        int fwd, bwd;
        if (x > 0) {
            fwd = x;
            bwd = x - 2 * left;
        }
        if (x < 0) {
            fwd = 2 * right - x;
            bwd = -x;
        }
        cout << fwd << ' ' << bwd << '\n';
    }
    return 0;
}
