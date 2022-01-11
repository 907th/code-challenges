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
    int n, k;
    cin >> n >> k;
    int sum_b = 0;
    int sum_g = 0;
    for (int i = 0; i < n; i++) {
        int b, g;
        cin >> b >> g;
        sum_b += b;
        sum_g += g;
    }
    int x = sum_b + k - (n + 1) * 2 - sum_g;
    if (x >= 0) cout << x << '\n';
    else cout << "Big Bang!\n";
    return 0;
}
