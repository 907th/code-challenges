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

// You will get the next formula after doing
// math calculations on paper with pen:
// Answer = 2 * S^2 - Di * Dj (for all pairs: i = 1..n, j = 1..n),
// where S = D0 + D1 + ..  + Dn, and Di - length of i-th road.
// It proves that the answer doesn't depend on the order of roads!
int main() {
    const int N = 100000;
    int n;
    int d[N];
    cin >> n;
    long long sum = 0;
    for (int i = 0; i < n; i++) {
        cin >> d[i];
        sum += d[i];
    }
    long long res = 0;
    for (int i = 0; i < n; i++) {
        res += sum * d[i];
        sum -= d[i];
        res += sum * d[i];
    }
    cout << res << '\n';
    return 0;
}
