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

void print(const vector<int> &v) {
    for (int i = 0; i < v.size(); i++) {
        cout << v[i];
        cout << (i < v.size() - 1 ? ' ' : '\n');
    }
}
int main() {
    int n, m;
    cin >> n >> m;

    // Solve
    vector<int> a, b;
    for (int i = 0; i < n; i++) a.push_back(i + 1);
    for (int i = 0; i < m; i++) b.push_back(n * (i + 1) + 1);

    // Checks
    for (int i = 0; i < a.size(); i++)
        for (int j = 0; j < a.size(); j++)
            if (i != j) assert(a[i] != a[j]);
    for (int i = 0; i < b.size(); i++)
        for (int j = 0; j < b.size(); j++)
            if (i != j) assert(b[i] != b[j]);
    for (int i = 0; i < a.size(); i++)
        for (int j = 0; j < b.size(); j++)
            assert(a[i] != b[j]);
    for (int i1 = 0; i1 < a.size(); i1++)
        for (int i2 = 0; i2 < a.size(); i2++)
            for (int j1 = 0; j1 < b.size(); j1++)
                for (int j2 = 0; j2 < b.size(); j2++)
                    if (i1 != i2 && j1 != j2)
                        assert(a[i1] + b[j1] != a[i2] + b[j2]);

    // Output
    print(a);
    print(b);
    return 0;
}
