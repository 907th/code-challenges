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

const int N = 100000;

long long a[N];
long long m[N];

void precalc() {
    a[0] = 0; m[0] = 0;
    a[1] = 1; m[1] = 1;
    for (int i = 2; i < N; i++) {
        if (i & 1) a[i] = a[i / 2] + a[i / 2 + 1];
        else a[i] = a[i / 2];
        assert(a[i] > 0);
        m[i] = max(m[i - 1], a[i]);
    }
}

int main() {
    precalc();
    while (1) {
        int n;
        cin >> n;
        if (n == 0) break;
        cout << m[n] << '\n';
    }
    return 0;
}
