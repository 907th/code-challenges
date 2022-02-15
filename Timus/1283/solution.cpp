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
    const double EPS = 1e-7;
    double a, b, p;
    cin >> a >> b >> p;
    int ans = 0;
    while (a > b + EPS) {
        double x = (a * p) / 100.0;
        a -= x;
        ans++;
    }
    cout << ans << '\n';
    return 0;
}
