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

int main() {
    int n;
    vector<int> d;
    cin >> n;
    for (int i = 0; i < n; i++) {
        int weight;
        cin >> weight;
        d.push_back(weight);
    }
    int ans = 1 << 30;
    for (int mask = 0; mask < (1 << n); mask++) {
        int sum[2];
        sum[0] = sum[1] = 0;
        for (int i = 0; i < n; i++) sum[(mask & (1 << i)) > 0 ? 1 : 0] += d[i];
        ans = min(ans, abs(sum[1] - sum[0]));
    }
    cout << ans << '\n';
    return 0;
}
