#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <string>
#include <cstdio>
#include <cmath>
#include <cstring>

using namespace std;

int main() {
    int n;
    cin >> n;
    long long d[50][2]; // last color: 0 - red, 1 - white
    memset(d, 0, 50 * 2 * sizeof(long long));
    d[1][0] = 1;
    d[1][1] = 1;
    for (int i = 1; i < n; i++) {
        // Add red/white stripe
        d[i + 1][1] += d[i][0];
        d[i + 1][0] += d[i][1];
        // Add blue + red/white stripe
        d[i + 2][1] += d[i][0];
        d[i + 2][0] += d[i][1];
    }
    cout << (d[n][0] + d[n][1]) << '\n';
    return 0;
}
