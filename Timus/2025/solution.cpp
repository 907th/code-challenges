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

int solve(int n, int k) {
    int res = n * n;
    int d = n / k;
    int r = n % k;
    res -= d * d * (k - r);
    res -= (d + 1) * (d + 1) * r;
    return res / 2;
}

// It can be shown that the sizes of all teams must be equal (as much as possible).
// Having Ci - size of team i, the answer is SUM(Ci * (N - Ci)) / 2 or (N^2 - SUM(Ci^2)) / 2.
// Lemma: if A and B - two numbers, then A^2 + B^2 is better (less) than (A-1)^2 + (B+1)^2 only when A > B.
// So we can always optimize solution while there exist Ci = A and Cj = B and A > B.
int main() {
    int t;
    cin >> t;
    for (int i = 0; i < t; i++) {
        int n, k;
        cin >> n >> k;
        cout << solve(n, k) << '\n';
    }
    return 0;
}
