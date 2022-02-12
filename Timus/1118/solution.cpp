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

bool is_prime(int n) {
    for (long long i = 2; i * i <= n; i++)
        if (n % i == 0) return false;
    return true;
}

double triviality(int n) {
    if (n == 1) return 0.0; // !!!
    double sum = 1.0;
    for (long long i = 2; i * i <= n; i++)
        if (n % i == 0) {
            sum += i;
            if (n / i != i) sum += n / i;
        }
    return sum / n;
}

int solve(int l, int r) {
    if (l == 1) return 1;
    int ans = r;
    double triv = triviality(ans);
    for (int n = r - 1; n >= l; n--) {
        double cur = triviality(n);
        if (cur < triv) {
            ans = n;
            triv = cur;
        }
        if (is_prime(n)) break;
    }
    return ans;
}

int main() {
    int l, r;
    cin >> l >> r;
    assert(l <= r);
    cout << solve(l, r) << '\n';
    return 0;
}
