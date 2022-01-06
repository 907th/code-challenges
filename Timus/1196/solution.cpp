#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <unordered_map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>
#include <cassert>

using namespace std;

int main() {
    unordered_map<int, bool> h;
    int n;
    scanf("%d", &n);
    h.reserve(n);
    for (int i = 0; i < n; i++) {
        int y;
        scanf("%d", &y);
        h[y] = true;
    }

    int res = 0;
    int m;
    scanf("%d", &m);
    for (int i = 0; i < m; i++) {
        int y;
        scanf("%d", &y);
        auto t = h.find(y);
        if (t != h.end()) res++;
    }

    cout << res << '\n';
    return 0;
}
