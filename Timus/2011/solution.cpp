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
#include <cctype>
#include <cassert>

using namespace std;

long long fact(int n) {
    long long ans = 1;
    for (int i = 1; i <= n; i++) ans *= i;
    return ans;
}

int main() {
    int n;
    cin >> n;
    int a, b, c;
    a = b = c = 0;
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v;
        if (v == 1) a++;
        if (v == 2) b++;
        if (v == 3) c++;
    }
    a = min(a, 5);
    b = min(b, 5);
    c = min(c, 5);
    long long ans = fact(a + b + c) / fact(a) / fact(b) / fact(c);
    cout << (ans >= 6 ? "Yes\n" : "No\n");
    return 0;
}
