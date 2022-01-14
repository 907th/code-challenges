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
    long long n, m;
    cin >> n >> m;
    long long x = min(n, m) - 1;
    long long res = 2LL * x;
    if (n > m) res++;
    cout << res << '\n';
    return 0;
}
