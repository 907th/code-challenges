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

int main() {
    const double PI = 3.1415926535;
    const double G = 10;
    double v, a, k;
    cin >> v >> a >> k;
    double alpha = a * PI / 180;
    double ans = (sin(2 * alpha) * v * v / G) * (k / (k - 1));
    cout << fixed << setprecision(2) << ans << '\n';
    return 0;
}
