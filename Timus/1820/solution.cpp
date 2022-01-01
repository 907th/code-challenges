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
    int res;
    if (n < k) {
        res = 2;
    } else {
        res = (n / k) * 2;
        if (n % k != 0) res += 2 * (n % k) > k ? 2 : 1;
    }
    cout << res << '\n';
    return 0;
}
