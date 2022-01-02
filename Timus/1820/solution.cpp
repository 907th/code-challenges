#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cstdio>
#include <cmath>

using namespace std;

int main() {
    int n, k;
    cin >> n >> k;
    int res = max(2, (2 * n + k - 1) / k);
    cout << res << '\n';
    return 0;
}
