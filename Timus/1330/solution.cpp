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
    int n;
    int s[10000];
    cin >> n;
    for (int i = 0; i < n; i++) cin >> s[i];

    int a[10000];
    a[0] = s[0];
    for (int i = 1; i < n; i++) a[i] = a[i - 1] + s[i];

    int q;
    cin >> q;
    for (int i = 0; i < q; i++) {
        int l, r;
        cin >> l >> r;
        l--; r--;
        cout << a[r] - (l > 0 ? a[l - 1] : 0) << '\n';
    }

    return 0;
}
