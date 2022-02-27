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

typedef unsigned long long ULL;

bool no_mult_overflow(ULL a, ULL b) {
    return ULLONG_MAX / b >= a;
}

void halt(bool flag) {
    cout << (flag ? "Yes\n" : "No\n");
    exit(0);
}

void check(int sum, ULL n, ULL p) {
    if (sum > 20) halt(false);
    if (sum == 20 && n > 1) halt(false);
    if (sum == 20 && n == 1) halt(true);
    if (sum < 20 && n == 1) halt(false);
    if (sum < 20 && n > 1) {
        ULL m = 1;
        while (sum < 20) {
            sum++;
            assert(no_mult_overflow(m, p));
            m *= p;
            if (m > n) halt(false);
        }
    }
}

int main() {
    ULL n;
    cin >> n;
    int sum = 0;
    while (n % 2 == 0) {
        sum++;
        n /= 2;
        check(sum, n, 2);
    }
    for (ULL i = 3; no_mult_overflow(i, i) && i * i <= n; i += 2) {
        check(sum, n, i);
        while (n % i == 0) {
            sum++;
            n /= i;
            check(sum, n, i);
        }
    }
    if (n > 1) sum++;
    halt(sum == 20);
    return 0;
}
