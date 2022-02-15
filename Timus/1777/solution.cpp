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

typedef long long LL;

// It is easy to show that at each algo step,
// the minimum difference between array values
// becomes at least twice smaller.
int main() {
    vector<LL> v;
    for (int i = 0; i < 3; i++) {
        LL x;
        cin >> x;
        v.push_back(x);
    }
    int ans = 0;
    while (1) {
        ans++;
        LL diff = 1LL << 62;
        for (int i = 0; i < v.size(); i++)
            for (int j = i + 1; j < v.size(); j++)
                diff = min(diff, llabs(v[i] - v[j]));
        if (diff == 0) break;
        v.push_back(diff);
    }
    cout << ans << '\n';
    return 0;
}
