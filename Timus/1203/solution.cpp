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

struct Report {
    int s, f;
};

bool report_cmp(const Report &a, const Report &b) {
    if (a.f != b.f) return a.f < b.f;
    return a.s < b.s;
}

int main() {
    const int N = 100000;
    int n;
    vector<Report> t;
    cin >> n;
    for (int i = 0; i < n; i++) {
        int s, f;
        cin >> s >> f;
        t.push_back(Report{s, f});
    }
    sort(t.begin(), t.end(), report_cmp);
    int k = 0;
    int end = t[0].f;
    for (int i = 1; i < n; i++) {
        if (t[i].s > end) {
            k++;
            end = t[i].f;
        }
    }
    k++;
    cout << k << '\n';
    return 0;
}
