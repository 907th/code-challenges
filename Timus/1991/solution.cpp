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
    int n, k;
    cin >> n >> k;
    int left = 0;
    int alive = 0;
    for (int i = 0; i < n; i++) {
        int x;
        cin >> x;
        if (x < k) {
            alive += k - x;
        } else {
            left += x - k;
        }
    }
    cout << left << ' ' << alive << '\n';
    return 0;
}
