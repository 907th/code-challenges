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
    int n, k;
    cin >> n >> k;
    int ok = 1;
    int ans = 0;
    while (ok < n && ok < k) {
        ok *= 2;
        ans++;
    }
    if (ok < n) {
        ans += (n - ok + k - 1) / k;
    }
    cout << ans << '\n';
    return 0;
}
