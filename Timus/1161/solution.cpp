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
#include <cctype>
#include <cassert>

using namespace std;

int main() {
    int n;
    vector<int> w;
    cin >> n;
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v;
        w.push_back(v);
    }
    sort(w.begin(), w.end(), greater<int>());
    double ans = w[0];
    for (int i = 1; i < n; i++) {
        ans = 2 * sqrt(ans * w[i]);
    }
    cout << fixed << setprecision(2);
    cout << ans << '\n';
    return 0;
}
