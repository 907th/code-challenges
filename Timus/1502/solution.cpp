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
    cin >> n;
    int sum = 0;
    long long ans = 0;
    for (int i = n; i >= 0; i--) {
        sum += i;
        ans += i * (n - i + 1);
        ans += sum;
    }
    cout << ans << '\n';
    return 0;
}
