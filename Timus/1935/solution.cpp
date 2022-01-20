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
    const int N = 100;
    int n, d[N];
    cin >> n;
    for (int i = 0; i < n; i++) cin >> d[i];
    sort(d, d + n);
    reverse(d, d + n);
    int ans = d[0];
    for (int i = 0; i < n; i++) ans += d[i];
    cout << ans << '\n';
    return 0;
}
