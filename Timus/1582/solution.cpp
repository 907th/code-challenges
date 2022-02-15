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

// All outcomes must be equal.
int main() {
    const double EPS = 1e-9;
    const double N = 1000.0;
    double k1, k2, k3;
    cin >> k1 >> k2 >> k3;
    int ans = (int) ((N * k1 * k2 * k3) / (k1 * k3 + k2 * k3 + k1 * k2) + 0.5 + EPS);
    cout << ans << '\n';
    return 0;
}
