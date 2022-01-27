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
    vector<int> p;
    cin >> n;
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v;
        p.push_back(v);
    }
    int min = 0;
    int sum = 0;
    int ans = 0;
    for (int i = 0; i < n; i++) {
        sum += p[i];
        min = std::min(min, sum);
        ans = std::max(ans, sum - min);
    }
    cout << ans << '\n';
    return 0;
}
