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
    int w, p, a, b;
    cin >> w >> p >> a >> b;
    int ans;
    if (a == b) {
        ans = w;
    } else if (a < b) {
        int k = b - a - 1;
        ans = 2 * p + k * (w + 2 * p);
    } else {
        int k = a - b - 1;
        ans = 2 * (w + p) + k * (w + 2 * p);
    }
    cout << ans << '\n';
    return 0;
}
