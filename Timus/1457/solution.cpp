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
    int n;
    int p[1000];
    cin >> n;
    for (int i = 0; i < n; i++) cin >> p[i];

    long long a = n;
    long long b = 0;
    for (int i = 0; i < n; i++) b -= p[i];
    b *= 2;

    // Base point of parabola:
    double ans = (double) -b / 2.0 / a;

    cout.setf(ios::fixed);
    cout.precision(6);
    cout << ans << '\n';

    return 0;
}
