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
    cin >> n;
    int p[101];
    memset(p, 0, sizeof(p));
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v;
        p[v - 600]++;
    }
    int res = 0;
    for (auto v : p) res += v / 4;
    cout << res << '\n';
    return 0;
}
