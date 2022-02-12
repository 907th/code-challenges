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
    int n, s;
    int m[20];
    cin >> n >> s;
    for (int i = 0; i < n; i++) cin >> m[i];

    int d[20];
    for (int i = 0; i < n; i++) {
        assert(s >= m[i]);
        d[i] = s / m[i];
        s /= d[i];
    }

    for (int i = 0; i < n; i++) {
        cout << (d[i] - 1);
        cout << (i < n - 1 ? ' ' : '\n');
    }
    return 0;
}
