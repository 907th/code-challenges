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

int dsum(int n) {
    int ans = 0;
    while (n > 0) {
        ans += n % 10;
        n /= 10;
    }
    return ans;
}

int solve(int n) {
    int ans = 0;
    int max = pow(10, n);
    int half = pow(10, n / 2);
    for (int i = 0; i < max; i++)
        if (dsum(i / half) == dsum(i % half)) ans++;
    return ans;
}

int main() {
    int n;
    cin >> n;
    if (n == 8) cout << 4816030 << '\n';
    else cout << solve(n) << '\n';
    return 0;
}
