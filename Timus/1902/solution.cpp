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
    int n;
    double t, s;
    int f[100];
    cin >> n >> t >> s;
    for (int i = 0; i < n; i++) {
        double f;
        cin >> f;
        double ans = (f + t + s) / 2.0;
        cout << fixed << setprecision(6) << ans << '\n';
    }
    return 0;
}
