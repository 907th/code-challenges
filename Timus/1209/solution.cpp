#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

const double EPS = 1e-12;

int solve(int a) {
    long long k = (-1.0 + sqrt(1.0 + 8.0 * (double) a)) / 2.0 + EPS;
    while (k * (k + 1) / 2 >= a) k--;
    return a == (k * (k + 1) / 2 + 1) ? 1 : 0;
}

int main() {
    int n;
    cin >> n;
    for (int i = 0; i < n; i++) {
        int a;
        cin >> a;
        cout << solve(a);
        cout << (i < n - 1 ? ' ' : '\n');
    }
    return 0;
}
