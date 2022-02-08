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

typedef pair<double, double> PDD;

const double PI = acos(-1.0);

double circle_length(double r) {
    return 2.0 * PI * r;
}

double distance(const PDD &a, const PDD &b) {
    return sqrt(
        (a.first - b.first) * (a.first - b.first) +
        (a.second - b.second) * (a.second - b.second)
    );
}

int main() {
    int n;
    double r;
    cin >> n >> r;
    vector<PDD> p;
    for (int i = 0; i < n; i++) {
        double x, y;
        cin >> x >> y;
        p.push_back(make_pair(x, y));
    }
    double ans = 0.0;
    for (int i = 0; i < n; i++) ans += distance(p[i], p[(i + 1) % n]);
    ans += circle_length(r);
    cout << fixed << setprecision(2) << ans << '\n';
    return 0;
}
