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
    int m[101];
    cin >> n;
    for (int i = 0; i < n; i++) cin >> m[i];
    sort(m, m + n);
    int res = 0;
    for (int i = 0; i <= n / 2; i++) res += m[i] / 2 + 1;
    cout << res << '\n';
    return 0;
}
